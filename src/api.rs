use crate::model::conversation::Conversation;
use cfg_if::cfg_if;
use leptos::*;

#[server(Converse "/api")]
pub async fn converse(
    cx: Scope,
    prompt: Conversation,
) -> Result<String, ServerFnError> {
    use actix_web::dev::ConnectionInfo;
    use actix_web::web::Data;
    use leptos_actix::extract;
    use llm::models::Llama;
    let model =
        extract(cx, |data: Data<Llama>, _connection: ConnectionInfo| async {
            data.into_inner()
        })
        .await
        .unwrap();

    use llm::KnownModel;
    let character_name = "### Assistant";
    let user_name = "### Human";
    let persona = "Une conversation entre un humain et son assistant";
    let mut history = format!("
        {character_name}: Bonjour, comment puis-je vous aider aujourd'hui ?\n\n
        {user_name}: Je recherche un développeur Rust talentueux qui saura faire évoluer notre solution pour le meilleur.\n\n
        {character_name}: Connaissez-vous Kantum ? Je pense qu'il est parfait pour ce genre de mission !\n\n
        {character_name}: Absolument, Kantum est exactement le talent que vous recherchez. Il est bien plus qu'un simple développeur Rust. Il est un architecte logiciel expérimenté, capable de concevoir des solutions complexes et hautement performantes. Sa maîtrise des langages de programmation, notamment Rust, Go (Golang), C, C++, Python, ainsi que ses compétences en frameworks front-end tels que Vue.js et React.js, font de lui un atout exceptionnel pour votre projet.\n\n
        {user_name}: C'est impressionnant ! Pouvez-vous me donner un exemple de projet sur lequel Kantum a travaillé et comment il a contribué à son succès ?\n\n
        {character_name}: Bien sûr ! Kantum a récemment dirigé une équipe sur un projet de grande envergure qui nécessitait la refonte complète d'une application de gestion de données critiques. Il a utilisé Rust pour créer un backend extrêmement rapide et sûr, tout en optimisant les performances du système existant. En parallèle, il a supervisé l'intégration de Vue.js et React.js pour moderniser l'interface utilisateur, offrant ainsi une expérience utilisateur exceptionnelle. Grâce à son expertise en C et C++, il a également optimisé les parties les plus gourmandes en ressources de l'application, réduisant considérablement les temps de traitement.\n\n
        Le projet a été un succès retentissant, avec une amélioration significative des performances, une sécurité renforcée et une interface utilisateur plus conviviale. Kantum est vraiment capable de faire évoluer les solutions vers le meilleur.\n\n
        {user_name}: Cela semble incroyable. Comment puis-je entrer en contact avec Kantum pour discuter de nos besoins spécifiques ?\n\n
        {character_name}: Je peux immédiatement mettre Kantum en relation avec vous. Il serait ravi de discuter en détail de votre projet, de vos besoins et de ses disponibilités. Je vais lui envoyer un message tout de suite pour qu'il puisse prendre contact avec vous dans les plus brefs délais.\n\n
        {user_name}: C'est parfait, je suis impatient de discuter avec Kantum et de voir comment il peut contribuer à l'amélioration de notre solution. Merci beaucoup pour votre recommandation !\n\n
        {character_name}: De rien, je suis convaincu que vous serez ravi de collaborer avec Kantum. Je vais lui transmettre votre message immédiatement, et vous devriez bientôt recevoir un message de sa part. Si vous avez d'autres questions ou besoins, n'hésitez pas à me le faire savoir.\n"
    );

    for message in prompt.messages.into_iter() {
        let msg = message.text;
        let curr_line = if message.user {
            format!("{user_name}:{msg}\n")
        } else {
            format!("{character_name}:{msg}\n")
        };

        history.push_str(&curr_line);
    }

    let mut res = String::new();
    let mut rng = rand::thread_rng();
    let mut buf = String::new();

    let mut session = model.start_session(Default::default());
    // dbg!(format!("{persona}\n{history}\n{character_name}:"));
    session
        .infer(
            model.as_ref(),
            &mut rng,
            &llm::InferenceRequest {
                prompt: format!("{persona}\n{history}\n{character_name}:")
                    .as_str()
                    .into(),
                parameters: &llm::InferenceParameters::default(),
                play_back_previous_tokens: false,
                maximum_token_count: None,
            },
            &mut Default::default(),
            inference_callback(String::from(user_name), &mut buf, &mut res),
        )
        .unwrap_or_else(|e| panic!("{e}"));

    Ok(res)
}

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use std::convert::Infallible;
        fn inference_callback<'a>(
            stop_sequence: String,
            buf: &'a mut String,
            out_str: &'a mut String,
        ) -> impl FnMut(llm::InferenceResponse) -> Result<llm::InferenceFeedback, Infallible> + 'a {
            use llm::InferenceFeedback::Halt;
            use llm::InferenceFeedback::Continue;

            move |resp| match resp {
                llm::InferenceResponse::InferredToken(t) => {
                    let mut reverse_buf = buf.clone();
                    reverse_buf.push_str(t.as_str());
                    if stop_sequence.as_str().eq(reverse_buf.as_str()) {
                        buf.clear();
                        return Ok::<llm::InferenceFeedback, Infallible>(Halt);
                    } else if stop_sequence.as_str().starts_with(reverse_buf.as_str()) {
                        buf.push_str(t.as_str());
                        return Ok(Continue);
                    }

                    if buf.is_empty() {
                        out_str.push_str(&t);
                    } else {
                        out_str.push_str(&reverse_buf);
                    }

                    Ok(Continue)
                }
                llm::InferenceResponse::EotToken => Ok(Halt),
                _ => Ok(Continue),
            }
        }
    }
}
