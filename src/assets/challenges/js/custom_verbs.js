// Access the data from window.konnektoren.challenge.data or use default data for testing
const data = window.konnektoren ? window.konnektoren.challenge.data :
    new Map([
        ["verb", "sein"],
        ["verb_translation", "to be"],
        ["questions", [
            new Map([
                ["id", "q1"],
                ["pronoun", "Ich"],
                ["answer", "bin"]
            ]),
            new Map([
                ["id", "q2"],
                ["pronoun", "Du"],
                ["answer", "bist"]
            ])
        ]]
    ]);


// Set the verb name and translation
document.getElementById('verb-name').textContent = data.get("verb");
document.getElementById('verb-translation').textContent = data.get("verb_translation");

// Generate the questions dynamically
function generateQuestions() {
    const questionsContainer = document.getElementById('questions-container');
    console.log(data.get("questions"));
    data.get("questions").forEach(question => {
        const questionDiv = document.createElement('div');
        questionDiv.classList.add('question');

        const label = document.createElement('label');
        label.setAttribute('for', question.get("id"));
        label.textContent = `${question.get("pronoun")} `;

        const input = document.createElement('input');
        input.type = 'text';
        input.id = question.get("id");
        input.placeholder = 'verb';

        label.appendChild(input);
        label.insertAdjacentText('beforeend', ` (${data.get("verb")})`);

        questionDiv.appendChild(label);
        questionsContainer.appendChild(questionDiv);
    });
}

generateQuestions();

function checkAnswers() {
    let correct = 0;
    const totalQuestions = data.get("questions").length;

    data.get("questions").forEach(question => {
        console.log(question);
        console.log(question.get("id"));

        const correctAnswer = question.get("answer").toLowerCase();
        const userAnswer = document.getElementById(question.get("id")).value.trim().toLowerCase();
        const answerField = document.getElementById(question.get("id"));
        let correctAnswerElement = document.querySelector(`#${question.get("id")} + .correct-answer`);

        // If the correct answer message already exists, remove it first
        if (correctAnswerElement) {
            correctAnswerElement.remove();
        }

        if (userAnswer === correctAnswer) {
            correct++;
            answerField.style.borderColor = 'green';
        } else {
            answerField.style.borderColor = 'red';
            // Show the correct answer
            const span = document.createElement('span');
            span.classList.add('correct-answer');
            span.textContent = ` (Correct: ${question.get("answer")})`;
            span.style.color = 'green';
            answerField.insertAdjacentElement('afterend', span);
        }
    });

    const result = document.getElementById('result');
    if (correct === totalQuestions) {
        result.textContent = 'Great job! All answers are correct.';
        result.style.color = 'green';
    } else {
        result.textContent = `You got ${correct} out of ${totalQuestions} correct. Check the correct answers shown next to your mistakes.`;
        result.style.color = 'red';
    }
}

function finishChallenge() {
    if (window.konnektoren && window.konnektoren.sendEvent) {
        const performance = calculatePerformance();
        const event = {
            type: "Finish",
            result: {
                id: window.konnektoren.challenge.id,
                performance: performance,
                data: collectUserAnswers()
            }
        };
        window.konnektoren.sendEvent(event);
    } else {
        // For testing purposes, display the results
        alert("Challenge Finished! Your performance: " + (calculatePerformance() * 100).toFixed(2) + "%");
    }
}

function calculatePerformance() {
    let correct = 0;
    const totalQuestions = data.get("questions").length;

    data.get("questions").forEach(question => {
        const correctAnswer = question.get("answer").toLowerCase();
        const userAnswer = document.getElementById(question.get("id")).value.trim().toLowerCase();
        if (userAnswer === correctAnswer) {
            correct++;
        }
    });

    return correct / totalQuestions;
}

function collectUserAnswers() {
    const userAnswers = [];

    data.get("questions").forEach(question => {
        const correctAnswer = question.get("answer");
        const userAnswer = document.getElementById(question.get("id")).value.trim();
        userAnswers.push({
            questionId: question.get("id"),
            userAnswer: userAnswer,
            correctAnswer: correctAnswer,
            isCorrect: userAnswer.toLowerCase() === correctAnswer.toLowerCase()
        });
    });

    return {
        answers: userAnswers
    };
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
