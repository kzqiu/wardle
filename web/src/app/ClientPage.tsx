'use client'
import '@picocss/pico/css/pico.min.css';
import React, {useState} from "react";
import Stack from "react-bootstrap/Stack";
import {InputForm} from "@/app/InputForm";
import {GuessHistory} from "@/app/GuessHistory";
import {ResultPopup} from "@/app/ResultPopup";

export const ClientPage = () => {
    const [showPopup, setShowPopup] = useState(false);
    const [guesses, setGuesses] = useState<Guess[]>([]);

    return (
        <div className={"container h-screen"}>
            <Stack direction="vertical" gap={3}>
                <InputForm setShowPopup={setShowPopup} guesses={guesses} setGuesses={setGuesses}/>
                <GuessHistory guesses={guesses}/>
                {showPopup ? <ResultPopup/> : null}
            </Stack>
        </div>
    );
}