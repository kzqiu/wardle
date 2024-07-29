type GuessHistoryProps = {
    guesses: Guess[];
}

const GuessCard = ({guess}: {guess: Guess}) => {
    return (
        <li>
            <p>{guess.name}</p>
            <ul>
                {guess.belligerentsInCommon.map((belligerent, index) => (
                    // TODO: Display the icon
                    <li key={index}>
                        <p>{belligerent.name}</p>
                    </li>
                ))}
            </ul>
        </li>
    );
}

export const GuessHistory = ({
                                    guesses,
                             }: GuessHistoryProps) => {

    return (
        <div>
        <h2>Guess History</h2>
        <ul>
            {guesses.map((guess, index) => (
                <li key={index}>
                    <GuessCard guess={guess}/>
                </li>
            ))}
        </ul>
        </div>
    );
}