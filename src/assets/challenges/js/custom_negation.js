// Access the data from window.konnektoren.challenge.data or use default data for testing
const data = window.konnektoren && window.konnektoren.challenge && window.konnektoren.challenge.data
    ? window.konnektoren.challenge.data
    : new Map([
        ["questions", [
            // Default data for testing purposes
            new Map([
                ["id", 1],
                ["sentence", "Ich habe _______ Hund."],
                ["correct_answer", "kein"],
                ["translation", "I don't have a dog."],
                ["explanation", "Use 'kein' to negate a noun with no article."],
                ["highlight", "Hund"],
                ["example", "dog"]
            ]),
            // ... Add more default questions as needed
        ]]
    ]);

// Function to generate questions dynamically
function generateQuestions() {
    const exerciseContainer = document.getElementById('exercise-container');
    const questionsArray = data.get("questions"); // Get the array of question Maps

    questionsArray.forEach(questionMap => {
        const id = questionMap.get("id");
        const sentence = questionMap.get("sentence");
        const correctAnswer = questionMap.get("correct_answer");
        const translation = questionMap.get("translation");
        const explanation = questionMap.get("explanation");
        const highlight = questionMap.get("highlight");
        const example = questionMap.get("example");

        const questionDiv = document.createElement('div');
        questionDiv.classList.add('question');

        // Create the question paragraph
        const questionP = document.createElement('p');
        questionP.innerHTML = `${id}. ${sentence} (${translation})`;
        questionDiv.appendChild(questionP);

        // Create the input element
        const input = document.createElement('input');
        input.type = 'text';
        input.id = `answer${id}`;
        input.placeholder = `Enter '${correctAnswer}'`;
        input.addEventListener('input', () => checkAnswer(id, correctAnswer));
        questionDiv.appendChild(input);

        // Feedback div
        const feedbackDiv = document.createElement('div');
        feedbackDiv.id = `feedback${id}`;
        feedbackDiv.classList.add('feedback');
        questionDiv.appendChild(feedbackDiv);

        // Explanation div
        const explanationDiv = document.createElement('div');
        explanationDiv.id = `explanation${id}`;
        explanationDiv.classList.add('explanation');
        explanationDiv.innerHTML = `${explanation} (e.g., <span class="highlight">'${highlight}'</span> - ${example}).`;
        questionDiv.appendChild(explanationDiv);

        // Append the question div to the exercise container
        exerciseContainer.appendChild(questionDiv);
    });
}

// Function to check answers
function checkAnswer(questionNumber, correctAnswer) {
    const userAnswer = document.getElementById(`answer${questionNumber}`).value.trim().toLowerCase();
    const feedbackElement = document.getElementById(`feedback${questionNumber}`);

    if (userAnswer === correctAnswer.toLowerCase()) {
        feedbackElement.innerHTML = `<span style="color: green;">Correct! The answer is <strong>${correctAnswer}</strong>.</span>`;
    } else if (userAnswer === '') {
        feedbackElement.innerHTML = ''; // Clear feedback if input is empty
    } else {
        feedbackElement.innerHTML = `<span style="color: red;">Incorrect. The correct answer is <strong>${correctAnswer}</strong>.</span>`;
    }
}

// Function to collect results and display summary
function finishExercise() {
    const questionsArray = data.get("questions");
    let totalQuestions = questionsArray.length;
    let correctAnswers = 0;

    questionsArray.forEach(questionMap => {
        const id = questionMap.get("id");
        const correctAnswer = questionMap.get("correct_answer").toLowerCase();
        const userAnswer = document.getElementById(`answer${id}`).value.trim().toLowerCase();

        if (userAnswer === correctAnswer) {
            correctAnswers++;
        }
    });

    const resultSummary = document.getElementById('result-summary');
    resultSummary.textContent = `You got ${correctAnswers} out of ${totalQuestions} correct.`;

    // Optionally, style the result summary based on performance
    if (correctAnswers === totalQuestions) {
        resultSummary.style.color = 'green';
        resultSummary.textContent += ' Excellent work!';
    } else {
        resultSummary.style.color = '#333';
    }

    // Disable the finish button after completion
    const finishButton = document.getElementById('finish-button');
    finishButton.disabled = true;
    finishButton.textContent = 'Completed';

    // Send event if konnektoren is available
    if (window.konnektoren && window.konnektoren.sendEvent) {
        const performance = correctAnswers / totalQuestions;
        const event = {
            type: "Finish",
            result: {
                id: window.konnektoren.challenge.id,
                performance: performance,
                data: collectUserAnswers()
            }
        };
        window.konnektoren.sendEvent(event);
    }
}

// Function to collect user answers
function collectUserAnswers() {
    const answers = [];
    const questionsArray = data.get("questions");

    questionsArray.forEach(questionMap => {
        const id = questionMap.get("id");
        const correctAnswer = questionMap.get("correct_answer");
        const userAnswer = document.getElementById(`answer${id}`).value.trim();

        answers.push({
            questionId: id,
            userAnswer: userAnswer,
            correctAnswer: correctAnswer,
            isCorrect: userAnswer.toLowerCase() === correctAnswer.toLowerCase()
        });
    });

    return { answers: answers };
}

// Add event listener to finish button
document.getElementById('finish-button').addEventListener('click', finishExercise);

// Call the function to generate questions on page load
generateQuestions();
