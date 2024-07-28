import React, {useState} from 'react';
import {Button} from "react-bootstrap";
import { Typeahead } from 'react-bootstrap-typeahead';
import {Option} from "react-bootstrap-typeahead/types/types";
import './InputForm.css';

type InputFormProps = {
    setShowPopup: (showPopup: boolean) => void;
}

export const InputForm = ({
                              setShowPopup,
                          }: InputFormProps) => {
    const [selected, setSelected] = useState<Option[]>([]);

    const handleSubmit = () => {
        if (selected.length === 0) {
            return;
        }
        console.log(selected[0]);

        // TODO: Implement the logic to check if the selected name is correct
    }

    const handleGiveUp = () => {
        setShowPopup(true);

    }

  return (
    <form>
      <label>
        Name of war:
        <Typeahead options={["War of the Austrian Succession", "American Revolutionary War", "World War Two"]} selected={selected} onChange={setSelected} />
      </label>
        <div className="button-container">
            <Button variant="primary" onClick={handleSubmit}>
                Submit
            </Button>
            <Button variant="secondary" onClick={handleGiveUp}>
                Give Up
            </Button>
        </div>
    </form>
  );
}