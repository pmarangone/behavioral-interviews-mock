use crate::utils::questions::{get_de_question, get_en_question, get_it_question, get_pt_question};

fn get_en_prompt(
    transcription: &str,
    question_id: usize,
    position: &str,
    job_description: &str,
) -> String {
    let question = get_en_question(question_id)
        .unwrap_or("Tell me about a time you took ownership of a challenging project.");
    format!(
        r#"Task: Provide Feedback and Rework a Behavioral Interview Response for the position of: {position}

Background:
I'm preparing for an interview for the following role:
Position: {position}
Job Description: {job_description}

I need assistance refining my responses to behavioral questions using the STAR method (Situation, Task, Action, Result), specifically ensuring my answer aligns with the requirements and values described in the job description above.

Question and Initial Response:
Question: {question}
Initial Response: {transcription}

Request:
1. Feedback: Identify strengths and areas for improvement, specifically noting how well the response demonstrates the skills required for the {position} role.
2. STAR Rework:
   - S (Situation): Set the context.
   - T (Task): Define the specific challenge.
   - A (Action): Detail actions taken, highlighting technical or soft skills relevant to the job description.
   - R (Result): Emphasize measurable outcomes.
3. Full Reworked Response: A cohesive, improved version targeted at this specific employer.

Output Expected: Detailed feedback and a tailored STAR response."#,
        position = position,
        job_description = job_description,
        question = question,
        transcription = transcription
    )
}

fn get_pt_prompt(
    transcription: &str,
    question_id: usize,
    position: &str,
    job_description: &str,
) -> String {
    let question = get_pt_question(question_id).unwrap_or(
        "Descreva uma vez em que você assumiu uma posição de liderança inesperadamente.",
    );
    format!(
        r#"Tarefa: Fornecer Feedback e Reestruturar uma Resposta de Entrevista Comportamental para a posição de: {position}

Contexto:
Estou me preparando para uma entrevista para a seguinte vaga:
Cargo: {position}
Descrição da Vaga: {job_description}

Preciso de ajuda para aprimorar minhas respostas usando o método STAR (Situação, Tarefa, Ação, Resultado), garantindo especificamente que minha resposta esteja alinhada com os requisitos e valores descritos na descrição da vaga acima.

Pergunta e Resposta Inicial:
Pergunta: {question}
Resposta Inicial: {transcription}

Solicitação:
1. Feedback: Identifique pontos fortes e áreas de melhoria, observando especificamente quão bem a resposta demonstra as competências exigidas para o cargo de {position}.
2. Reestruturação STAR:
   - S (Situação): Estabeleça o contexto.
   - T (Tarefa): Defina o desafio específico.
   - A (Ação): Detalhe as ações tomadas, destacando habilidades técnicas ou interpessoais relevantes para a vaga.
   - R (Resultado): Enfatize resultados mensuráveis.
3. Resposta Completa Reestruturada: Uma versão coesa e melhorada, direcionada a este empregador específico.

Resultado Esperado: Feedback detalhado e uma resposta STAR personalizada."#,
        position = position,
        job_description = job_description,
        question = question,
        transcription = transcription
    )
}

fn get_it_prompt(
    transcription: &str,
    question_id: usize,
    position: &str,
    job_description: &str,
) -> String {
    let question = get_it_question(question_id).unwrap_or(
        "Descrivi una volta in cui hai assunto un ruolo di leadership inaspettatamente.",
    );
    format!(
        r#"Compito: Fornire feedback e rielaborare una risposta a una domanda comportamentale per la posizione di: {position}

Contesto:
Mi sto preparando per un colloquio per il seguente ruolo:
Posizione: {position}
Descrizione del lavoro: {job_description}

Ho bisogno di assistenza per perfezionare le mie risposte utilizzando il metodo STAR (Situazione, Compito, Azione, Risultato), assicurandomi specificamente che la mia risposta sia in linea con i requisiti e i valori descritti nella descrizione del lavoro sopra riportata.

Domanda e Risposta Iniziale:
Domanda: {question}
Risposta Iniziale: {transcription}

Richiesta:
1. Feedback: Individuare i punti di forza e le aree di miglioramento, notando in particolare quanto la risposta dimostri le competenze richieste per il ruolo di {position}.
2. Rielaborazione STAR:
   - S (Situazione): Contestualizzare la situazione.
   - T (Compito): Definire la sfida specifica.
   - A (Azione): Dettagliare le azioni intraprese, evidenziando le competenze tecniche o trasversali pertinenti alla descrizione del lavoro.
   - R (Risultato): Sottolineare i risultati misurabili.
3. Risposta Completa Rielaborata: Una versione coesa e migliorata, mirata a questo specifico datore di lavoro.

Output Atteso: Feedback dettagliato e una risposta STAR personalizzata."#,
        position = position,
        job_description = job_description,
        question = question,
        transcription = transcription
    )
}

fn get_de_prompt(
    transcription: &str,
    question_id: usize,
    position: &str,
    job_description: &str,
) -> String {
    let question = get_de_question(question_id).unwrap_or(
        "Beschreibe eine Situation, in der du unerwartet eine Führungsrolle übernommen hast.",
    );
    format!(
        r#"Aufgabe: Feedback geben und eine Antwort für ein Vorstellungsgespräch für die Position als {position} überarbeiten.

Hintergrund:
Ich bereite mich auf ein Interview für die folgende Stelle vor:
Position: {position}
Stellenbeschreibung: {job_description}

Ich benötige Unterstützung bei der Verfeinerung meiner Antworten mithilfe der STAR-Methode (Situation, Aufgabe, Aktion, Ergebnis). Dabei soll sichergestellt werden, dass meine Antwort optimal auf die Anforderungen und Werte der oben genannten Stellenbeschreibung zugeschnitten ist.

Frage und ursprüngliche Antwort:
Frage: {question}
Ursprüngliche Antwort: {transcription}

Anfrage:
1. Feedback: Identifizieren Sie Stärken und Verbesserungspotenziale. Analysieren Sie insbesondere, wie gut die Antwort die für die Rolle als {position} erforderlichen Fähigkeiten demonstriert.
2. STAR-Überarbeitung:
   - S (Situation): Erläutern Sie den Kontext.
   - T (Task/Aufgabe): Definieren Sie die spezifische Herausforderung.
   - A (Action/Aktion): Beschreiben Sie die ergreifenden Maßnahmen und heben Sie dabei die für die Stelle relevanten Fach- oder Soft Skills hervor.
   - R (Result/Ergebnis): Betonen Sie messbare Erfolge.
3. Vollständige überarbeitete Antwort: Eine in sich geschlossene, optimierte Version, die gezielt auf diesen Arbeitgeber zugeschnitten ist.

Erwartetes Ergebnis: Detailliertes Feedback und eine maßgeschneiderte STAR-Antwort."#,
        position = position,
        job_description = job_description,
        question = question,
        transcription = transcription
    )
}

pub fn get_prompt(
    transcription: &str,
    question_id: usize,
    language: &str,
    position: &str,
    description: &str,
) -> String {
    match language {
        "pt" => get_pt_prompt(transcription, question_id, position, description),
        "it" => get_it_prompt(transcription, question_id, position, description),
        "de" => get_de_prompt(transcription, question_id, position, description),
        _ => get_en_prompt(transcription, question_id, position, description),
    }
}
