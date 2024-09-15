// Access the data from window.konnektoren.challenge.data or use default data for testing
const data = window.konnektoren && window.konnektoren.challenge && window.konnektoren.challenge.data
    ? window.konnektoren.challenge.data
    : new Map([
        ["questions", [
            new Map([
                ["scenario", "Ich habe ______ (Montag) frei."],
                ["options", ["am Montag", "Montag", "im Montag"]],
                ["correctAnswer", "am Montag"],
                ["hint", "The correct preposition for days is 'am'."]
            ]),
            new Map([
                ["scenario", "Der Kurs beginnt ______ (9 Uhr)."],
                ["options", ["um 9 Uhr", "am 9 Uhr", "im 9 Uhr"]],
                ["correctAnswer", "um 9 Uhr"],
                ["hint", "For specific times, 'um' is the correct preposition."]
            ])
        ]]
    ]);

// Initialize variables
let currentQuestionIndex = 0;
let score = 0;
const questions = data.get("questions");
const totalQuestions = questions.length;

// Function to shuffle options
function shuffleOptions(options) {
    for (let i = options.length - 1; i > 0; i--) {
        const j = Math.floor(Math.random() * (i + 1));
        [options[i], options[j]] = [options[j], options[i]];
    }
    return options;
}

// Function to load the current question
function loadQuestion() {
    if (currentQuestionIndex >= totalQuestions) {
        finishChallenge();
        return;
    }

    const question = questions[currentQuestionIndex];
    const scenarioText = question.get("scenario");
    const options = shuffleOptions([...question.get("options")]);
    const hint = question.get("hint");

    // Update scenario text
    document.getElementById('scenario-text').textContent = scenarioText;

    // Populate the select dropdown
    const timeNounSelect = document.getElementById('time-noun');
    timeNounSelect.innerHTML = '<option value="">Select</option>'; // Reset options

    options.forEach(option => {
        const opt = document.createElement('option');
        opt.value = option;
        opt.textContent = option;
        timeNounSelect.appendChild(opt);
    });

    // Update hint
    const hintBox = document.getElementById('hint-box');
    hintBox.textContent = `💡 Hint: ${hint}`;
    hintBox.classList.remove('hidden');

    // Reset feedback and styles
    const feedback = document.getElementById('feedback');
    feedback.textContent = '';
    timeNounSelect.style.backgroundColor = '';

    // Enable the select dropdown
    timeNounSelect.disabled = false;
}

// Function to check the user's answer
function checkAnswer() {
    const question = questions[currentQuestionIndex];
    const selectedOption = document.getElementById('time-noun').value;
    const correctAnswer = question.get("correctAnswer");

    const feedbackElement = document.getElementById('feedback');
    const timeNounSelect = document.getElementById('time-noun');

    if (selectedOption === "") {
        feedbackElement.textContent = "Bitte wählen Sie eine Option aus.";
        feedbackElement.style.color = "#d32f2f"; // Red color
        return;
    }

    if (selectedOption === correctAnswer) {
        feedbackElement.textContent = "Richtig! Gut gemacht.";
        feedbackElement.style.color = "#388e3c"; // Green color
        score += 10;
        document.getElementById('score').textContent = `Score: ${score}`;
        timeNounSelect.style.backgroundColor = "#c8e6c9"; // Light green

        // Update progress bar
        updateProgressBar();

        // Move to next question after a short delay
        setTimeout(() => {
            currentQuestionIndex++;
            loadQuestion();
        }, 1000);
    } else {
        feedbackElement.textContent = `Falsch. Die richtige Antwort ist "${correctAnswer}".`;
        feedbackElement.style.color = "#d32f2f"; // Red color
        timeNounSelect.style.backgroundColor = "#ffcdd2"; // Light red
    }
}

// Function to update the progress bar
function updateProgressBar() {
    const progress = ((currentQuestionIndex + 1) / totalQuestions) * 100;
    document.getElementById('progress-bar').style.width = `${progress}%`;
}

// Finish function to summarize results and send them
function finishChallenge() {
    // Hide the question elements
    document.getElementById('scenario-container').classList.add('hidden');
    document.querySelector('.options').classList.add('hidden');
    document.getElementById('feedback').classList.add('hidden');
    document.getElementById('hint-box').classList.add('hidden');
    document.getElementById('time-noun').disabled = true;

    // Show the result summary
    const resultSummary = document.getElementById('result-summary');
    resultSummary.textContent = `Sie haben ${score} von ${totalQuestions * 10} Punkten erreicht!`;
    resultSummary.classList.remove('hidden');

    // Show the Finish button
    const finishBtn = document.getElementById('finish-btn');
    finishBtn.classList.remove('hidden');

    // Add event listener to Finish button
    finishBtn.addEventListener('click', () => {
        // Optionally, you can redirect or perform another action here
        alert("Danke fürs Üben!");
    });

    // Send the result event if konnektoren is available
    if (window.konnektoren && window.konnektoren.sendEvent) {
        const performance = score / (totalQuestions * 10); // Performance as a fraction
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
        alert(`Challenge Finished! Your performance: ${(score / (totalQuestions * 10) * 100).toFixed(2)}%`);
    }
}

// Function to collect user answers
function collectUserAnswers() {
    const answers = [];

    questions.forEach((question, index) => {
        const selectedOption = document.getElementById('time-noun').value; // Assuming answers are processed in order
        answers.push({
            questionNumber: index + 1,
            scenario: question.get("scenario"),
            selectedOption: selectedOption || 'Keine Auswahl',
            correctAnswer: question.get("correctAnswer"),
            isCorrect: selectedOption === question.get("correctAnswer")
        });
    });

    return { answers: answers };
}

// Event listener for the select dropdown
document.getElementById('time-noun').addEventListener('change', checkAnswer);

loadQuestion();
