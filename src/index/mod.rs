use actix_web::{HttpResponse, HttpRequest, Error};
use tera::{Tera, Context};
use futures::{future::ok, Future};


const ERROR_500: &str = "
<html>
    <head>
        <style>
            :root {
                font-size: 20px;
                font-family: 'IBM Plex Mono';
                line-height: 1.5;
                color: rgba(255, 255, 255, 0.25);
                }

                body {
                height: 100vh;
                display: flex;
                flex-direction: column;
                justify-content: center;
                align-items: center;
                text-align: center;
                background: #333333;
                }

                a {
                color: white;
                display: inline;
                }

                #error {
                margin-bottom: 1rem;
                font-size: 2rem;
                font-weight: 500;
                text-transform: uppercase;
                letter-spacing: 0.075em;
                color: #C94D4D;
                -webkit-animation: pulse 4s infinite alternate;
                        animation: pulse 4s infinite alternate;
                position: relative;
                }
                @-webkit-keyframes pulse {
                from {
                    opacity: 0.5;
                }
                50% {
                    opacity: 0.5;
                }
                }
                @keyframes pulse {
                from {
                    opacity: 0.5;
                }
                50% {
                    opacity: 0.5;
                }
                }
                #error::before {
                content: '';
                width: 0.75rem;
                height: 50vh;
                margin-bottom: 0.75em;
                position: absolute;
                left: 50%;
                bottom: 100%;
                -webkit-transform: translateX(-50%);
                        transform: translateX(-50%);
                background: linear-gradient(to bottom, rgba(255, 255, 255, 0.1) 60%, transparent 100%);
                }

                #desc {
                margin: 2em 0 1em;
                }

                .error-num, .error-num__clip, .error-num__clip::before, .error-num__clip-left {
                position: relative;
                font-size: 10rem;
                font-family: 'Oswald';
                letter-spacing: -0.01em;
                color: white;
                -webkit-animation: colorSplit 1.25s steps(2, end) infinite;
                        animation: colorSplit 1.25s steps(2, end) infinite;
                }
                @-webkit-keyframes colorSplit {
                25% {
                    text-shadow: -0.02em 0 0 #ED008C, 0.025em 0 0 #0087EF;
                }
                75% {
                    text-shadow: -0.035em 0 0 #ED008C, 0.04em 0 0 #0087EF;
                }
                }
                @keyframes colorSplit {
                25% {
                    text-shadow: -0.02em 0 0 #ED008C, 0.025em 0 0 #0087EF;
                }
                75% {
                    text-shadow: -0.035em 0 0 #ED008C, 0.04em 0 0 #0087EF;
                }
                }
                .error-num__clip, .error-num__clip::before, .error-num__clip-left {
                position: absolute;
                top: 0;
                left: -2px;
                z-index: 10;
                color: #333;
                overflow: visible;
                -webkit-clip-path: polygon(0% 0%, 100% 0, 100% 25%, 0 25%, 0 30%, 100% 30%, 100% 50%, 0 50%, 0 60%, 100% 60%, 100% 65%, 0 65%, 0 80%, 100% 80%, 100% 85%, 0 85%, 0% 0%);
                        clip-path: polygon(0% 0%, 100% 0, 100% 25%, 0 25%, 0 30%, 100% 30%, 100% 50%, 0 50%, 0 60%, 100% 60%, 100% 65%, 0 65%, 0 80%, 100% 80%, 100% 85%, 0 85%, 0% 0%);
                -webkit-animation: glitch 1s steps(2, start) infinite;
                        animation: glitch 1s steps(2, start) infinite;
                }
                @-webkit-keyframes glitch {
                30% {
                    left: 0;
                }
                to {
                    left: 0;
                }
                }
                @keyframes glitch {
                30% {
                    left: 0;
                }
                to {
                    left: 0;
                }
                }
                .error-num__clip::before, .error-num__clip-left::before {
                content: '500';
                left: 0.05em;
                color: white;
                z-index: 9;
                -webkit-clip-path: polygon(0% 0%, 100% 0, 100% 26%, 0 26%, 0 29%, 100% 29%, 100% 51%, 0 51%, 0 59%, 100% 59%, 100% 66%, 0 66%, 0 79%, 100% 79%, 100% 86%, 0 86%, 0% 0%);
                        clip-path: polygon(0% 0%, 100% 0, 100% 26%, 0 26%, 0 29%, 100% 29%, 100% 51%, 0 51%, 0 59%, 100% 59%, 100% 66%, 0 66%, 0 79%, 100% 79%, 100% 86%, 0 86%, 0% 0%);
                }

        </style>
    </head>
    <div id='error'>{{ error }}</div>
    <div class='error-num'>500
    <div class='error-num__clip'>500</div>
    </div>
    <p id='desc'>{{ text_error }}</p>
</html>

";


pub fn index(_req: HttpRequest) -> impl Future<Item = HttpResponse, Error = Error> {
    let mut tera = Tera::default();
    let mut context = Context::new();  
    
    context.insert("error", "Error");
    context.insert("text_error", "Uh oh, there seems to be a problem.");

    tera.add_raw_template("error500", ERROR_500).expect("Error loading \"error 500\" template");

    tera.add_template_file("src/ng/dist/index.html", Some("index")).unwrap_or(());

    let rendered = tera.render("index", &context).unwrap_or_else(|_err|{
        tera.render("error500", &context).expect("Error render \"error 500\" page.")}
        );
    
    ok(HttpResponse::Ok()
            .content_type("text/html")
            .body(rendered))
}

// use actix_web::{fs::NamedFile, HttpRequest, Error, Result};
// use share::common::AppState;

// pub fn index(_req: &HttpRequest<AppState>) -> Result<NamedFile> {
//     Ok(NamedFile::open("public/index.html")?)
// }