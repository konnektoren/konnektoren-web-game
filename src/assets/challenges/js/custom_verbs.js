const data = challenge.data;

function checkAnswers() {
    const answers = data.answers;

    let correct = 0;

    for (let key in answers) {
        const userAnswer = document.getElementById(key).value.trim().toLowerCase();
        const answerField = document.getElementById(key);
        let correctAnswerElement = document.querySelector(`#${key} + .correct-answer`);

        // If the correct answer message already exists, remove it first
        if (correctAnswerElement) {
            correctAnswerElement.remove();
        }

        if (userAnswer === answers[key]) {
            correct++;
            answerField.style.borderColor = 'green';
        } else {
            answerField.style.borderColor = 'red';
            // Create a new span to show the correct answer next to the wrong one
            const span = document.createElement('span');
            span.classList.add('correct-answer');
            span.textContent = ` (Correct: ${answers[key]})`;
            span.style.color = 'green';
            answerField.insertAdjacentElement('afterend', span);
        }
    }

    const result = document.getElementById('result');
    if (correct === Object.keys(answers).length) {
        result.textContent = 'Great job! All answers are correct.';
        result.style.color = 'green';
    } else {
        result.textContent = `You got ${correct} out of 6 correct. Check the correct answers shown next to your mistakes.`;
        result.style.color = 'red';
    }
}

function finishChallenge() {
    if (window.emit_challenge_event) {
        const event = { type: "Finish", result: {} };
        window.emit_challenge_event(event);
    }
}

document.getElementById("finish-button").addEventListener("click", function () {
    const finishButton = document.getElementById("finish-button");

    if (finishButton.textContent === "Check Answers") {
        checkAnswers();
        finishButton.textContent = "End";
    } else if (finishButton.textContent === "End") {
        finishChallenge()
    }
});