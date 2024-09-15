console.log(window.konnektoren);

const data = window.konnektoren ? window.konnektoren.challenge.data :
    new Map([
        ["options", [
            new Map([["id", 0], ["name", "der"]]),
            new Map([["id", 1], ["name", "die"]]),
            new Map([["id", 2], ["name", "das"]])
        ]],
        ["questions", [
            new Map([
                ["question", "What is the article for 'Haus'?"],
                ["option", 2], // 'das' is correct
                ["help", "The article for 'Haus' is 'das'."]
            ]),
            new Map([
                ["question", "What is the article for 'Tisch'?"],
                ["option", 0], // 'der' is correct
                ["help", "The article for 'Tisch' is 'der'."]
            ]),
            new Map([
                ["question", "What is the article for 'Buch'?"],
                ["option", 2], // 'das' is correct
                ["help", "The article for 'Buch' is 'das'."]
            ]),
        ]]
    ]);

console.log(data);

let currentQuestionIndex = 0;
let correctAnswers = 0;
let userAnswers = [];

function finishChallenge() {
    if (window.konnektoren && window.konnektoren.sendEvent) {
        const event = {
            type: "Finish",
            result: {
                id: window.konnektoren.challenge.id,
                performance: calculatePerformance(),
                data: {
                    answers: userAnswers
                }
            }
        };
        window.konnektoren.sendEvent(event);
    } else {
        // For testing purposes, display the results
        console.log("Quiz Finished! Your performance: " + (calculatePerformance() * 100) + "%");
        console.log("User Answers:", userAnswers);
        alert("Quiz Finished! Your performance: " + (calculatePerformance() * 100) + "%");
    }
}

// Load the current question
function loadQuestion() {

    const currentQuestion = data.get("questions")[currentQuestionIndex];
    const questionText = document.getElementById('question-text');

    if(questionText === null) {
        return;
    }

    const helpText = document.getElementById('help-text');
    const questionImage = document.getElementById('question-image');
    const optionsList = document.getElementById('options-list');
    const feedback = document.getElementById('feedback');

    // Reset feedback on new question
    feedback.textContent = '';
    feedback.style.display = 'none';

    // Update question and help text
    questionText.textContent = currentQuestion.get("question");
    helpText.textContent = currentQuestion.get("help");

    // Update question image if available
    if (currentQuestion.get("image")) {
        questionImage.className = "fas " + currentQuestion.get("image");
    } else {
        questionImage.className = "";
    }

    // Clear previous options
    optionsList.innerHTML = '';

    // Create and display options
    data.get("options").forEach(option => {
        const li = document.createElement('li');
        li.textContent = option.get("name");
        li.dataset.id = option.get("id");

        // Handle option click
        li.addEventListener('click', () => {
            checkAnswer(option.get("id"));
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
    const currentQuestion = data.get("questions")[currentQuestionIndex];
    const feedback = document.getElementById('feedback');
    let isCorrect = false; // Variable to track correctness

    // Show feedback based on whether the answer is correct or not
    if (selectedOption === currentQuestion.get("option")) {
        correctAnswers++; // Increment correct answers
        isCorrect = true; // Answer is correct
        feedback.textContent = 'Correct!';
        feedback.style.color = 'green';
    } else {
        feedback.textContent = 'Incorrect!';
        feedback.style.color = 'red';
    }

    // Record user's answer
    userAnswers.push({
        questionId: currentQuestion.id,
        selectedOption: selectedOption,
        correctOption: currentQuestion.option,
        isCorrect: isCorrect
    });

    feedback.style.display = 'block';

    if(currentQuestionIndex === data.get("questions").length - 1) {
        finishChallenge();
        return;
    } else {
        setTimeout(() => {
            nextQuestion();
        }, 1000);
    }
}

function calculatePerformance() {
    const totalQuestions = data.get("questions").length;
    const performance = correctAnswers / totalQuestions;
    return performance;
}

// Automatically load the first question on page load
loadQuestion();

const finishButton = document.getElementById("finish-button");

if (finishButton) {
    finishButton.addEventListener("click", function () {
        finishChallenge();
    });
}

function displayResults() {
    // Check if window.konnektoren and result data are available
    const result = window.konnektoren && window.konnektoren.result;

    if (!result) {
        console.warn('No result data available in window.konnektoren.result.');
        return; // Exit the function if no result data
    }

    // Access the challenge data (questions and options)
    const challengeData = window.konnektoren && window.konnektoren.challenge && window.konnektoren.challenge.data;

    if (!challengeData) {
        console.warn('No challenge data available in window.konnektoren.challenge.data.');
        return; // Exit the function if no challenge data
    }

    // Get the performance and answers
    const performance = result.performance;
    const answersMap = result.data.get("answers");

    // Try to get the performance element
    const performanceElement = document.getElementById('performance');

    if (performanceElement) {
        // Display performance percentage
        performanceElement.textContent = (performance * 100).toFixed(2);
    } else {
        console.warn('Element with id "performance" not found.');
        // Optionally, you can create the element if needed
    }

    // Try to get the results container element
    const resultsContainer = document.getElementById('results-container');

    if (resultsContainer) {
        // Access and convert the challenge data
        const questionsMap = challengeData.get("questions"); // Array of Maps
        const optionsMap = challengeData.get("options");     // Array of Maps

        // Convert Map data to arrays of plain objects
        const questions = Array.from(questionsMap.entries()).map(([key, qMap]) => {
            return {
                id: key,
                question: qMap.get("question"),
                option: qMap.get("option"),
                help: qMap.get("help")
            };
        });

        const options = optionsMap.map(oMap => {
            return {
                id: oMap.get("id"),
                name: oMap.get("name")
            };
        });

        const answers = Array.from(answersMap.entries()).map(([questionId, answerMap]) => {
            return {
                questionId: questionId,
                selectedOption: answerMap.get("selectedOption"),
                option: answerMap.get("option"),
                isCorrect: answerMap.get("isCorrect")
            };
        });

        // Display detailed results
        const resultsList = document.createElement('ul');

        answers.forEach((answer) => {
            const questionId = answer.questionId;

            const question = questions.find(q => q.id === questionId);
            const selectedOption = options.find(o => o.id === answer.selectedOption);
            const correctOption = options.find(o => o.id === question.option);

            const listItem = document.createElement('li');

            listItem.innerHTML = `
                <p><strong>Question:</strong> ${question ? question.question : 'Question not found'}</p>
                <p><strong>Your answer:</strong> ${selectedOption ? selectedOption.name : 'Option not found'} ${answer.isCorrect ? '✅' : '❌'}</p>
                <p><strong>Correct answer:</strong> ${correctOption ? correctOption.name : 'Option not found'}</p>
                <p><strong>Help:</strong> ${question ? question.help : 'Help not available'}</p>
            `;

            resultsList.appendChild(listItem);
        });

        resultsContainer.appendChild(resultsList);
    } else {
        console.warn('Element with id "results-container" not found.');
        // Optionally, you can create the element or display a message elsewhere
    }
}

// Call the function to display results
displayResults();
