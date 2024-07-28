'use client'
import '@picocss/pico/css/pico.min.css';
import React, {useState} from "react";
import Stack from "react-bootstrap/Stack";
import {InputForm} from "@/app/InputForm";
import {GuessHistory} from "@/app/GuessHistory";
import {ResultPopup} from "@/app/ResultPopup";


export const ClientPage = () => {
    const [showPopup, setShowPopup] = useState(false);

    return (
        <div className={"container full-height"}>
            <Stack direction="vertical" gap={3}>
                <InputForm setShowPopup={setShowPopup}/>
                <GuessHistory/>
                {showPopup ? <ResultPopup/> : null}
            </Stack>
        </div>
    );
}