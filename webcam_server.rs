// Importation des modules nécessaires depuis la bibliothèque opencv
use opencv::{
    core,
    highgui,
    imgcodecs,
    imgproc,
    prelude::*,
    videoio,
};

// Importation des macros et des modules nécessaires depuis la bibliothèque Rocket
use rocket::{get, routes};

// Importation du type Mutex depuis la bibliothèque standard de Rust
use std::sync::Mutex;

// Utilisation de la macro lazy_static pour créer une variable globale thread-safe
// qui stocke l'image annotée encodée en JPEG
lazy_static! {
    static ref ANNOTATED_FRAME: Mutex<Option<Vec<u8>>> = Mutex::new(None);
}

// Définition d'une route Rocket qui renvoie l'image annotée lorsqu'une requête est reçue à la racine ("/")
#[get("/")]
fn index() -> &'static [u8] {
    ANNOTATED_FRAME.lock().unwrap().as_deref().unwrap_or(&[])
}

// Fonction principale pour la capture et l'annotation des images de la webcam
fn main() {
    // Ouverture d'une capture vidéo à partir de la webcam par défaut (identifiant 0)
    let mut camera = videoio::VideoCapture::new(0, videoio::CAP_ANY).unwrap();
    let mut frame = Mat::default();

    loop {
        // Lecture d'une image depuis la webcam
        camera.read(&mut frame).unwrap();

        // Clonage de l'image pour l'annotation
        let mut annotated_frame = frame.clone();

        // Définition des paramètres pour le texte d'annotation
        let text = "Lieu: Salon, Date: 2023-05-30";
        let org = core::Point::new(50, 50);
        let font_face = imgproc::FONT_HERSHEY_SIMPLEX;
        let font_scale = 1.0;
        let color = core::Scalar::new(255.0, 255.0, 255.0, 0.0);
        let thickness = 2;

        // Ajout du texte d'annotation sur l'image
        imgproc::put_text(
            &mut annotated_frame,
            text,
            org,
            font_face,
            font_scale,
            color,
            thickness,
            imgproc::LINE_AA,
            false,
        )
        .unwrap();

        // Encodage de l'image annotée au format JPEG
        let mut buf = Vec::new();
        imgcodecs::imencode(".jpg", &annotated_frame, &mut buf, &vec![])
            .unwrap();

        // Stockage de l'image annotée encodée dans la variable globale ANNOTATED_FRAME
        let mut locked_frame = ANNOTATED_FRAME.lock().unwrap();
        *locked_frame = Some(buf);
    }
}

// Fonction pour construire l'application Rocket
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

// Point d'entrée principal de l'application
#[rocket::main]
async fn main_async() {
    // Lancement du thread de capture et d'annotation des images de la webcam
    let _ = std::thread::spawn(main);
    
    // Lancement de l'application Rocket
    rocket().launch().await.expect("Failed to launch Rocket");
}
