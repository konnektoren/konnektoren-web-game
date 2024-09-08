const data = window.konnektoren.challenge.data;

function checkAnswers() {
    const answers = data.get("answers");  // Assuming answers is a Map
    console.log(answers); // Map(6) ...

    let correct = 0;

    // Use Map's forEach to iterate over key-value pairs
    answers.forEach((correctAnswer, key) => {
        console.log(key);
        const userAnswer = document.getElementById(key).value.trim().toLowerCase();
        const answerField = document.getElementById(key);
        let correctAnswerElement = document.querySelector(`#${key} + .correct-answer`);

        // If the correct answer message already exists, remove it first
        if (correctAnswerElement) {
            correctAnswerElement.remove();
        }

        if (userAnswer === correctAnswer.toLowerCase()) {  // Case-insensitive comparison
            correct++;
            answerField.style.borderColor = 'green';
        } else {
            answerField.style.borderColor = 'red';
            // Create a new span to show the correct answer next to the wrong one
            const span = document.createElement('span');
            span.classList.add('correct-answer');
            span.textContent = ` (Correct: ${correctAnswer})`;
            span.style.color = 'green';
            answerField.insertAdjacentElement('afterend', span);
        }
    });

    const result = document.getElementById('result');
    if (correct === answers.size) {  // Use size for Map length
        result.textContent = 'Great job! All answers are correct.';
        result.style.color = 'green';
    } else {
        result.textContent = `You got ${correct} out of ${answers.size} correct. Check the correct answers shown next to your mistakes.`;
        result.style.color = 'red';
    }
}

function finishChallenge() {
    if (window.konnektoren.sendEvent) {
        const event = { type: "Finish", result: {} };
        window.konnektoren.sendEvent(event);
    }
}

document.getElementById("finish-button").addEventListener("click", function () {
    const finishButton = document.getElementById("finish-button");

    if (finishButton.textContent === "Check Answers") {
        checkAnswers();
        finishButton.textContent = "End";
    } else if (finishButton.textContent === "End") {
        finishChallenge();
    }
});
