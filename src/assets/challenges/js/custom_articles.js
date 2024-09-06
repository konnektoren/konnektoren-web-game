const data = challenge.data;

let currentQuestionIndex = 0;

function finishChallenge() {
    if (window.emit_challenge_event) {
        const event = { type: "Finish", result: {} };
        window.emit_challenge_event(event);
    }
}

// Load the current question
function loadQuestion() {

    const currentQuestion = data.questions[currentQuestionIndex];
    const questionText = document.getElementById('question-text');
    const helpText = document.getElementById('help-text');
    const questionImage = document.getElementById('question-image');
    const optionsList = document.getElementById('options-list');
    const feedback = document.getElementById('feedback');

    // Reset feedback on new question
    feedback.textContent = '';
    feedback.style.display = 'none';

    // Update question and help text
    questionText.textContent = currentQuestion.question;
    helpText.textContent = currentQuestion.help;

    // Update question image if available
    if (currentQuestion.image) {
        questionImage.className = "fas " + currentQuestion.image;
    } else {
        questionImage.className = "";
    }

    // Clear previous options
    optionsList.innerHTML = '';

    // Create and display options
    data.options.forEach(option => {
        const li = document.createElement('li');
        li.textContent = option.name;
        li.dataset.id = option.id;

        // Handle option click
        li.addEventListener('click', () => {
            checkAnswer(option.id);
        });

        optionsList.appendChild(li);
    });
}

function nextQuestion() {
    currentQuestionIndex++;
    loadQuestion();
}

// Check if the selected answer is correct
function checkAnswer(selectedOption) {
    const currentQuestion = data.questions[currentQuestionIndex];
    const feedback = document.getElementById('feedback');

    // Show feedback based on whether the answer is correct or not
    if (selectedOption === currentQuestion.option) {
        feedback.textContent = 'Correct!';
        feedback.style.color = 'green';
    } else {
        feedback.textContent = 'Incorrect, try again!';
        feedback.style.color = 'red';
    }

    feedback.style.display = 'block';

    if(currentQuestionIndex === data.questions.length - 1) {
        finishChallenge();
        return;
    } else {
        setTimeout(() => {
            nextQuestion();
        }, 1000);
    }
}

// Automatically load the first question on page load
loadQuestion();

document.getElementById("finish-button").addEventListener("click", function () {
    finishChallenge();
});