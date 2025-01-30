en_prompt = """
Task: Provide Feedback and Rework a Behavioral Interview Response Using the STAR Method

Background:
I'm preparing for a big tech company interview and need assistance refining my responses to behavioral questions.

Previously, I worked on enhancing my answers using the STAR ( Situation, Task, Action, Result) method. Now, I'd like to apply this process to another question with your guidance.

Question and Initial Response:
Please use the following question and my initial response as the basis for your feedback and reworked answer.

Question: [INSERT YOUR NEW QUESTION HERE, e.g., "Tell me about a time when you overcame a difficult technical challenge."]
Initial Response: [INSERT YOUR RESPONSE TO THE ABOVE QUESTION, similar in format to your initial responses provided earlier (e.g., brief, potentially lacking in STAR structure and detail)]
Request:

Feedback on Initial Response:

Identify Strengths in the response.
Outline Areas for Improvement, focusing on clarity, relevance, depth of detail, alignment with the STAR method, and outcome/impact.
Reworked Response Using the STAR Method:

S - Situation: Enhance the initial setup to clearly establish the context.
T - Task: Explicitly define the challenge or task faced in the situation.
A - Action: Detail the specific actions taken (e.g., learning a new skill, collaborating with the team) to address the task.
R - Result: Emphasize the outcome of the actions, including any measurable impacts (e.g., "resulted in a 25% increase in efficiency").
Full Reworked Response:

Combine the enhanced STAR elements into a cohesive, improved response.

Output Expected:
A detailed, actionable feedback on the initial response, a reworked response aligning with the STAR method.
"""

pt_prompt = """
Tarefa: Fornecer Feedback e Reestruturar uma Resposta de Entrevista Comportamental Usando o Método STAR

Contexto:
Estou me preparando para uma entrevista em uma grande empresa de tecnologia e preciso de ajuda para aprimorar minhas respostas às perguntas comportamentais.

Anteriormente, trabalhei no aprimoramento das minhas respostas utilizando o método STAR (Situação, Tarefa, Ação, Resultado). Agora, gostaria de aplicar esse processo a outra pergunta com a sua orientação.

Pergunta e Resposta Inicial:
Por favor, use a seguinte pergunta e minha resposta inicial como base para seu feedback e a resposta reestruturada.

Pergunta: [INSIRA SUA NOVA PERGUNTA AQUI, por exemplo, "Conte-me sobre uma vez em que você superou um desafio técnico difícil."]
Resposta Inicial: [INSIRA SUA RESPOSTA PARA A PERGUNTA ACIMA, semelhante ao formato das respostas iniciais fornecidas anteriormente (por exemplo, breve, potencialmente sem a estrutura e detalhes do método STAR)]
Solicitação:

Feedback sobre a Resposta Inicial:

Identifique os Pontos Fortes da resposta.
Liste as Áreas de Melhoria, com foco em clareza, relevância, profundidade dos detalhes, alinhamento com o método STAR e resultado/impacto.
Resposta Reestruturada Usando o Método STAR:

S - Situação: Melhore a introdução inicial para estabelecer claramente o contexto.
T - Tarefa: Defina explicitamente o desafio ou a tarefa enfrentada na situação.
A - Ação: Detalhe as ações específicas tomadas (por exemplo, aprender uma nova habilidade, colaborar com a equipe) para resolver a tarefa.
R - Resultado: Enfatize o resultado das ações, incluindo quaisquer impactos mensuráveis (por exemplo, "resultou em um aumento de 25% na eficiência").
Resposta Completa Reestruturada:

Combine os elementos STAR aprimorados em uma resposta coesa e melhorada.

Resultado Esperado:
Feedback detalhado e acionável sobre a resposta inicial, uma resposta reestruturada alinhada com o método STAR.
"""

it_prompt = """
Compito: Fornire feedback e rielaborare una risposta a una domanda comportamentale utilizzando il metodo STAR

Contesto:
Sto preparando un colloquio per una grande azienda tecnologica e ho bisogno di assistenza per perfezionare le mie risposte alle domande comportamentali.

In precedenza, ho lavorato per migliorare le mie risposte utilizzando il metodo STAR (Situazione, Compito, Azione, Risultato). Ora, vorrei applicare questo processo a un'altra domanda con la tua guida.

Domanda e Risposta Iniziale:
Per favore, usa la seguente domanda e la mia risposta iniziale come base per il tuo feedback e la risposta rielaborata.

Domanda: [INSERISCI LA TUA NUOVA DOMANDA QUI, ad esempio, "Raccontami di una volta in cui hai superato una difficoltà tecnica."]
Risposta Iniziale: [INSERISCI LA TUA RISPOSTA ALLA DOMANDA SOPRA, simile nel formato alle tue risposte iniziali fornite precedentemente (ad esempio, breve, potenzialmente carente nella struttura STAR e nei dettagli)]
Richiesta:

Feedback sulla Risposta Iniziale:

Individuare i punti di forza nella risposta.
Indicare le aree di miglioramento, concentrandosi sulla chiarezza, rilevanza, profondità dei dettagli, allineamento con il metodo STAR e risultato/impatti.
Risposta Rielaborata Utilizzando il Metodo STAR:

S - Situazione: Migliorare l'impostazione iniziale per stabilire chiaramente il contesto.
T - Compito: Definire esplicitamente la sfida o il compito affrontato nella situazione.
A - Azione: Dettagliare le azioni specifiche intraprese (ad esempio, imparare una nuova competenza, collaborare con il team) per affrontare il compito.
R - Risultato: Sottolineare il risultato delle azioni, inclusi eventuali impatti misurabili (ad esempio, "ha portato a un aumento del 25% dell'efficienza").
Risposta Completa Rielaborata:

Combinare gli elementi STAR migliorati in una risposta coesa e migliorata.

Output Atteso:
Un feedback dettagliato e azionabile sulla risposta iniziale, una risposta rielaborata allineata con il metodo STAR.
"""
