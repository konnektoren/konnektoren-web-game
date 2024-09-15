// Access the data from window.konnektoren.challenge.data or use default data for testing
const data = window.konnektoren && window.konnektoren.challenge && window.konnektoren.challenge.data
    ? window.konnektoren.challenge.data
    : new Map([
        ["questions", [
            // Default data for testing purposes
            new Map([
                ["id", "q1"],
                ["pronoun", "Ich"],
                ["sentence", "Ich _______ nach Hause gegangen."],
                ["options", ["bin", "habe"]],
                ["correct_answer", "bin"]
            ]),
            new Map([
                ["id", "q2"],
                ["pronoun", "Ich"],
                ["sentence", "Ich _______ ein Buch gelesen."],
                ["options", ["bin", "habe"]],
                ["correct_answer", "habe"]
            ]),
            // ... Add more default questions as needed
        ]]
    ]);

// Function to generate questions dynamically
function generateQuestions() {
    const questionsContainer = document.getElementById('questions-container');
    const questionsArray = data.get("questions"); // Get the array of questions

    questionsArray.forEach(question => {
        // Each question is a Map
        const questionDiv = document.createElement('div');
        questionDiv.classList.add('question');

        const id = question.get("id");
        const sentence = question.get("sentence");
        const options = question.get("options"); // options is an array

        const label = document.createElement('label');
        label.setAttribute('for', id);
        label.textContent = sentence;

        const select = document.createElement('select');
        select.id = id;

        // Default option
        const defaultOption = document.createElement('option');
        defaultOption.value = "";
        defaultOption.disabled = true;
        defaultOption.selected = true;
        defaultOption.textContent = "Select";
        select.appendChild(defaultOption);

        // Add options from question data
        options.forEach(optionValue => {
            const option = document.createElement('option');
            option.value = optionValue;
            option.textContent = optionValue;
            select.appendChild(option);
        });

        // Append select to label
        label.appendChild(select);

        // Append label to question div
        questionDiv.appendChild(label);

        // Append question div to container
        questionsContainer.appendChild(questionDiv);
    });
}

// Function to check answers
function checkAnswers() {
    let totalCorrect = 0;
    const questionsArray = data.get("questions"); // Get the array of questions
    const totalQuestions = questionsArray.length;

    questionsArray.forEach(question => {
        const id = question.get("id");
        const correctAnswer = question.get("correct_answer");
        const select = document.getElementById(id);
        const userAnswer = select.value;

        // Remove any previous feedback
        select.classList.remove('correct', 'incorrect');
        const existingFeedback = select.nextElementSibling;
        if (existingFeedback && existingFeedback.classList.contains('correct-answer')) {
            existingFeedback.remove();
        }

        if (userAnswer === correctAnswer) {
            totalCorrect++;
            select.classList.add('correct');
        } else {
            select.classList.add('incorrect');
            // Display the correct answer
            const span = document.createElement('span');
            span.classList.add('correct-answer');
            span.textContent = ` (Correct: ${correctAnswer})`;
            select.insertAdjacentElement('afterend', span);
        }
    });

    // Display the result
    const resultElement = document.getElementById('result');
    resultElement.textContent = `You got ${totalCorrect} out of ${totalQuestions} correct.`;
    if (totalCorrect === totalQuestions) {
        resultElement.style.color = '#28a745'; // Green color
        resultElement.textContent += ' Excellent work!';
    } else {
        resultElement.style.color = '#dc3545'; // Red color
    }
}

// Function to calculate performance
function calculatePerformance() {
    let totalCorrect = 0;
    const questionsArray = data.get("questions"); // Get the array of questions
    const totalQuestions = questionsArray.length;

    questionsArray.forEach(question => {
        const id = question.get("id");
        const correctAnswer = question.get("correct_answer");
        const userAnswer = document.getElementById(id).value;
        if (userAnswer === correctAnswer) {
            totalCorrect++;
        }
    });

    return totalCorrect / totalQuestions;
}

// Function to collect user answers
function collectUserAnswers() {
    const answers = [];
    const questionsArray = data.get("questions"); // Get the array of questions

    questionsArray.forEach(question => {
        const id = question.get("id");
        const correctAnswer = question.get("correct_answer");
        const userAnswer = document.getElementById(id).value;
        answers.push({
            questionId: id,
            userAnswer: userAnswer,
            correctAnswer: correctAnswer,
            isCorrect: userAnswer === correctAnswer
        });
    });

    return { answers: answers };
}

// Event listener for the submit button
document.getElementById('submit-btn').addEventListener('click', function () {
    const submitBtn = document.getElementById('submit-btn');

    if (submitBtn.textContent === "Check Answers") {
        checkAnswers();
        submitBtn.textContent = "Finish";
    } else if (submitBtn.textContent === "Finish") {
        finishChallenge();
    }
});

// Function to finish the challenge (for integration with konnektoren)
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
        // For testing purposes
        alert(`Challenge Finished! Your performance: ${(calculatePerformance() * 100).toFixed(2)}%`);
    }
}

// Call the function to generate questions on page load
generateQuestions();
