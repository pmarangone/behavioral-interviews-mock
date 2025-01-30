"use client";

const localhost = process.env.NEXT_PUBLIC_LOCALHOST;

import { useState, useEffect, useRef } from "react";
import { Play, Pause, RefreshCw } from "lucide-react";
import { marked } from "marked";

import CopyButton from "./button";

async function processText(question: string, transcription: string, language: string) {
  const formatted = JSON.stringify(
    {
      "question": question,
      "response": transcription
    }
  );

  const url = localhost + `chat?lang=${language}`

  const responseChat = await fetch(url, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: formatted,
  });

  if (!responseChat.ok) {
    throw new Error(`HTTP error! status: ${responseChat.status}`);
  }

  let responseText = await responseChat.json();
  responseText = transcription + "\n\n" + responseText["message"];


  return responseText;
}

async function processAudio(formData: FormData, language: string) {
  const response = await fetch(localhost + `transcribe?lang=${language}`, {
    method: "POST",
    body: formData,
  });

  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }

  let responseToQuestion = await response.json();
  console.log(Object.keys(responseToQuestion));

  responseToQuestion = responseToQuestion["transcript"];
  return responseToQuestion;
}

async function fetchQuestion(
  language: string,
  questionIndex?: number
): Promise<{ question: string; index: number }> {

  const url = questionIndex !== undefined
    ? `${localhost}get-random-question?lang=${language}&index=${questionIndex}`
    : `${localhost}get-random-question?lang=${language}`;

  const response: { question: string; index: number } = await fetch(url)
    .then((response) => response.json())
    .then((data) => {
      return { question: data.question, index: data.question_id }; // Extract question and index
    })
    .catch((error) => {
      console.error("Error fetching random question:", error);
      const mock = fetchMockText(); // Fallback mock question
      return { question: mock, index: -1 }; // Return mock with -1 as index
    });

  return response;
}


function fetchMockText() {
  return "Speak your mind, we're listening! (Not right now, try again later)";
}

export default function AudioRecorder() {
  const [isRecording, setIsRecording] = useState(false);
  const [time, setTime] = useState(0);
  const [audioURL, setAudioURL] = useState<string | null>(null);
  const [transcription, setTranscription] = useState<string | null>(null);
  const [isProcessing, setIsProcessing] = useState(false);
  const [displayText, setDisplayText] = useState(
    "Ready to record your thoughts? Refresh the question when you're ready"
  );
  const [questionIndex, setQuestionIndex] = useState(0);
  const [isRefreshing, setIsRefreshing] = useState(false);
  const [language, setLanguage] = useState("english"); // State for language
  const [showMessage, setShowMessage] = useState(false);
  const mediaRecorderRef = useRef<MediaRecorder | null>(null);
  const chunksRef = useRef<BlobPart[]>([]);

  const copyTranscription = () => {
    if (transcription) {
      navigator.clipboard
        .writeText(transcription)
        .then(() => {
          console.log("Transcription copied to clipboard");
        })
        .catch((err) => {
          console.error("Failed to copy text: ", err);
        });
    }
  };

  useEffect(() => {
    let interval: NodeJS.Timeout;
    if (isRecording) {
      interval = setInterval(() => {
        setTime((prev) => prev + 1);
      }, 1000);
    }
    return () => {
      if (interval) {
        clearInterval(interval);
      }
    };
  }, [isRecording]);

  const startRecording = async () => {
    try {
      const stream = await navigator.mediaDevices.getUserMedia({ audio: true });
      const mediaRecorder = new MediaRecorder(stream);
      mediaRecorderRef.current = mediaRecorder;
      chunksRef.current = [];

      mediaRecorder.ondataavailable = (e) => {
        chunksRef.current.push(e.data);
      };

      mediaRecorder.onstop = () => {
        const blob = new Blob(chunksRef.current, {
          type: "audio/ogg; codecs=opus",
        });
        setAudioURL(URL.createObjectURL(blob));
        processAudioData(blob);
      };

      mediaRecorder.start();
      setIsRecording(true);
    } catch (error) {
      console.error("Error accessing microphone:", error);
    }
  };

  const stopRecording = () => {
    if (mediaRecorderRef.current && isRecording) {
      mediaRecorderRef.current.stop();
      setIsRecording(false);
      setTime(0);
    }
  };

  const processAudioData = async (blob: Blob) => {
    setIsProcessing(true);
    const formData = new FormData();
    formData.append("file", blob, "recording.ogg");

    try {
      const result = await processAudio(formData, language); // Pass language here
      const chat_feedback = await processText(displayText, result, language);
      setTranscription(chat_feedback);
    } catch (error) {
      console.error("Error processing audio:", error);
      setTranscription("Error processing audio. Please try again.");
    } finally {
      setIsProcessing(false);
    }
  };

  const refreshText = async () => {
    setIsRefreshing(true);
    try {
      console.log('refresh language', language)
      const result = await fetchQuestion(language);
      setDisplayText(result.question);
      setQuestionIndex(result.index);
    } catch (error) {
      console.error("Error fetching new text:", error);
    } finally {
      setIsRefreshing(false);
    }
  };

  const handleLanguageChange = async (event: React.ChangeEvent<HTMLSelectElement>) => {
    const selectedLanguage = event.target.value;
    setLanguage(selectedLanguage);

    if (selectedLanguage === "italian") {
      setShowMessage(true);
      setTimeout(() => {
        setShowMessage(false);
      }, 500);
    }

    // Refresh the text with the newly selected language
    try {
      const result = await fetchQuestion(selectedLanguage, questionIndex);
      setDisplayText(result.question);
    } catch (error) {
      console.error("Error fetching new text:", error);
    }
  };

  const formatTime = (seconds: number) => {
    const minutes = Math.floor(seconds / 60);
    const remainingSeconds = seconds % 60;
    return `${minutes.toString().padStart(2, "0")}:${remainingSeconds
      .toString()
      .padStart(2, "0")}`;
  };

  return (
    <div className="flex flex-col items-center justify-center min-h-screen p-4 space-y-6">
      <div className="flex items-center space-x-4">
        <p className="text-xl font-semibold">{displayText}</p>
        <button
          onClick={refreshText}
          disabled={isRefreshing}
          className="p-2 bg-gray-200 rounded-full hover:bg-gray-300 focus:outline-none focus:ring-2 focus:ring-gray-400"
          aria-label="Refresh text"
        >
          <RefreshCw
            className={`w-5 h-5 ${isRefreshing ? "animate-spin" : ""}`}
          />
        </button>

        <select
          value={language}
          onChange={handleLanguageChange} // No need for `async` or `await` here
          className="p-2 bg-gray-200 rounded-full hover:bg-gray-300 focus:outline-none focus:ring-2 focus:ring-gray-400"
          aria-label="Select language"
        >
          <option value="english">English</option>
          <option value="portuguese">Portuguese</option>
          <option value="italian">Italian</option>
        </select>
        {showMessage && (
          <div className="mt-2 text-sm text-green-500">
            🤌🏻
          </div>
        )}
      </div>

      <button
        onClick={isRecording ? stopRecording : startRecording}
        className="w-24 h-24 rounded-full bg-[#4285f4] hover:bg-[#3367d6] transition-colors flex items-center justify-center focus:outline-none focus:ring-4 focus:ring-blue-300"
        aria-label={isRecording ? "Stop recording" : "Start recording"}
      >
        {isRecording ? (
          <Pause className="w-8 h-8 text-white" />
        ) : (
          <Play className="w-8 h-8 text-white ml-1" />
        )}
      </button>

      <div className="font-mono text-4xl" aria-live="polite">
        {formatTime(time)}
      </div>

      <div className="text-xl">
        Press the button to {isRecording ? "stop" : "start"} recording
      </div>

      {audioURL && (
        <div className="w-full max-w-md bg-gray-100 rounded-full p-2">
          <audio src={audioURL} controls className="w-full" />
        </div>
      )}

      {isProcessing && (
        <div className="text-xl font-semibold">Processing audio...</div>
      )}

      {transcription && (
        <div className="w-full max-w-3xl bg-gray-100 rounded-lg p-4 mt-4 overflow-auto max-h-[60vh]">
          <div className="flex justify-between items-center mb-2">
            <h2 className="text-lg font-semibold">Transcription:</h2>
            <CopyButton onClick={copyTranscription} />
          </div>
          <div
            className="prose prose-sm max-w-none"
            dangerouslySetInnerHTML={{ __html: marked(transcription) }}
          />
        </div>
      )}
    </div>
  );
}
