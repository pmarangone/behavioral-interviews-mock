# Mock Behavioral Interview with LLMs (in development)

## v0: First Steps
Since I was just building it for myself, the architecture was simple:

**Front-end**: React App

**Back-end**: FastAPI server with three endpoints:
- /random-question: returns a random behavioral question [hardcoded array].
- /transcribe: takes and audio file (ogg format) and returns the transcription.
- /chat: receives { question: _, response: _ } and returns { feedback: _ }.

**Machine learning**: 
- For the transcription, I used Whisper from OpenAI.
- For the chat, I used Gemma Chat which allowed me to iterate over my previous responses to the same question.

I was either running this on my machine or on Google Colab, using Ngrok to expose the server running on Colab through a tunnel.


## v1: Google AI API (GenAI) Free tier

Running the Gemma Chat on my machine/Colab wasn't a good approach then I switched to Google AI API free tier.

Kept the server architecture running on my on machine, with the same endpoints, but now:
- /chat: sends the question and response to GenAI api and receives a feedback.

## v2: Time to get feedback from friends

I decided to publish this repo and since I didn't want to let my computer receiving unknown requests all day, 
and Colab wasn't reliable for long periods, moved the server to a notebook on Kaggle. Ngrok kept doing
the job but the notebook session had a 12h limit session time (which, for my surprise, kept online for only two hours :smile_with_tear:).

Deployed the web app to Vercel but was still figuring out where to host my server. 

*Time to move to a more reliant infrastructure.*

## v3: Third-party services enters the chat 
Moved the server to a third party service, Railway, which hosted the question and feedback endpoints while the transcription endpoint was inside a docker still running the whisper model on my machine through a Fastapi endpoint and Ngrok to expose my local server.

At this point I figured out that I didn't need to have two servers and an Event-Based Architecture would be a better approach.

## v4: Updated the architecture to Event-Based Architecture (EDA)
Kept running the server on Railway with the same endpoints: question, transcribe, feedback and added Web Socket, such that the front-end would receive a task_id and open a connection to the server until the transcription was available.
With that, I mean:

1. The transcribe route creates a task, sends it to a queue (RabbitMQ) and return its id (uuid).

2. The consumer receives the message and process the transcription. The result is published to redis.

3. The web app opens a Web Socket to the server which opens a channel to Redis and keeps listening for any publications on that channel. 
When the task status is completed, the server returns the transcription and closes the WS connection to the web app.

With this, I removed the session limit from Kaggle (since I was running the Whisper model on my machine) and now I could more easily retry the transcription if something went wrong (more robust service).


## v5: Host everything on third-party services

The previous setup was:
- Run the main server on Railway
- Run RabbitMQ, Redis and the Whisper model in three containers on my machine.

This wasn't scalable nor it was safe for me to let it running all day for my friends to use it. 

I started using CloudAMQP to host my queue, Upstash for Redis and Hugging Face Inference API for the transcription. The limits of these services are high enough for me not to worry about for at least 20 people using it a few times day.

By doing this, I simplified the architecture since I didn't need to have a queue (HF Inference API is fast enough). 

Now, the /transcribe endpoint calls the HF endpoint. Everything's good.


## Wrapping Up:
Everything here could be just a wrapper of other services (which, for my surprise, didn't exist 5 years ago when I started learning Machine Learning. :very_happy_face:)

## Further improvements:
- Compare improvements: Store questions and transcriptions to database to keep track of responses to the same question. 
- Languages: Add support for more languages.
- Role: Add option to choose which role you're applying, i.e, Project Manager, Fullstack, etc...
- Add history such that the model keeps track of your previous responses and take that into account to provide a better feedback. Build a 'profile' of scores of response to tell the user when they improved and when they responses degraded.  


## Links
- Machine learning:
  - Whisper: https://huggingface.co/openai/whisper-large-v3-turbo
  - Gemma Chat (allows 'memory'): https://ai.google.dev/gemma/docs/gemma_chat
  - Hugging Face Inference API: https://huggingface.co/docs/api-inference/index
- Infrastructure:
  - Server hosting: https://railway.com/
  - Web app hosting: https://vercel.com/
  - Rabbitmq: https://cloudamqp.com/
  - Redis: https://upstash.com/