\const data = window.konnektoren && window.konnektoren.challenge && window.konnektoren.challenge.data ?
    window.konnektoren.challenge.data :
    new Map([
        ["questions", [
            {
                scenario: "Ich habe ______ (Montag) frei.",
                options: ["am Montag", "Montag", "im Montag"],
                correctAnswer: "am Montag",
                hint: "The correct preposition for days is 'am'."
            },
            {
                scenario: "Der Kurs beginnt ______ (9 Uhr).",
                options: ["um 9 Uhr", "am 9 Uhr", "im 9 Uhr"],
                correctAnswer: "um 9 Uhr",
                hint: "For specific times, 'um' is the correct preposition."
            },
            {
                scenario: "Ich arbeite seit ______ (zwei Monaten) hier.",
                options: ["zwei Monate", "zwei Monaten", "zwei Monat"],
                correctAnswer: "zwei Monaten",
                hint: "The preposition 'seit' takes the dative case."
            },
            {
                scenario: "Wir haben das Projekt ______ (letzten Monat) abgeschlossen.",
                options: ["im letzten Monat", "letzten Monat", "in letzten Monat"],
                correctAnswer: "letzten Monat",
                hint: "For past actions, 'letzten Monat' uses the accusative case without a preposition."
            },
            {
                scenario: "Sie wird am ______ (nächsten Samstag) heiraten.",
                options: ["nächsten Samstag", "nächster Samstag", "nächstem Samstag"],
                correctAnswer: "nächsten Samstag",
                hint: "For specific future days, use 'am' + the accusative case."
            },
            {
                scenario: "Er hat die Prüfung ______ (vor einem Jahr) bestanden.",
                options: ["vor einem Jahr", "vor einen Jahr", "in einem Jahr"],
                correctAnswer: "vor einem Jahr",
                hint: "The preposition 'vor' is followed by the dative case."
            },
            {
                scenario: "Ich komme ______ (in einer Stunde) zurück.",
                options: ["in einer Stunde", "in eine Stunde", "in einer Stunden"],
                correctAnswer: "in einer Stunde",
                hint: "'In' is followed by the dative case when referring to future time periods."
            }
        ]]
    ]);

const questions = data.get("questions");
let currentQuestionIndex = 0;
let score = 0;
const totalQuestions = questions.length;

function shuffleOptions(options) {
    for (let i = options.length - 1; i > 0; i--) {
        const j = Math.floor(Math.random() * (i + 1));
        [options[i], options[j]] = [options[j], options[i]];
    }
    return options;
}

function loadQuestion() {
    const question = questions[currentQuestionIndex];

    document.getElementById('scenario-text').textContent = question.scenario;

    const timeNounSelect = document.getElementById('time-noun');

    // Clear the current options and reset select dropdown
    timeNounSelect.innerHTML = '<option value="">Select</option>';

    // Shuffle and add options
    const shuffledOptions = shuffleOptions([...question.options]);
    shuffledOptions.forEach(option => {
        timeNounSelect.innerHTML += `<option value="${option}">${option}</option>`;
    });

    document.getElementById('hint-box').textContent = `💡 ${question.hint}`;
    document.getElementById('hint-box').classList.remove('hidden');

    // Reset feedback and styles
    document.getElementById('feedback').textContent = '';
    timeNounSelect.style.backgroundColor = '';

    // Add event listener to dropdown
    timeNounSelect.addEventListener('change', checkAnswer);
}

function checkAnswer() {
    const selectedOption = document.getElementById('time-noun').value;
    const question = questions[currentQuestionIndex];

    const correctAnswer = question.correctAnswer;

    let feedback = '';
    let feedbackColor = '';

    if (selectedOption === correctAnswer) {
        feedback = 'Correct! Well done.';
        feedbackColor = '#388e3c';
        score += 10;


        updateProgressBar();


        currentQuestionIndex = (currentQuestionIndex + 1) % totalQuestions;
        loadQuestion();
    } else {
        feedback = `Incorrect. The correct answer is "${correctAnswer}". ${question.hint}`;
        feedbackColor = '#d32f2f'; // Dark red
    }


    document.getElementById('feedback').textContent = feedback;
    document.getElementById('feedback').style.color = feedbackColor;
    document.getElementById('score').textContent = `Score: ${score}`;


    const timeNounSelect = document.getElementById('time-noun');

    timeNounSelect.querySelectorAll('option').forEach(option => {
        option.style.backgroundColor = option.value === correctAnswer ? '#388e3c' : '';
        if (option.value === selectedOption && option.value !== correctAnswer) {
            option.style.backgroundColor = '#d32f2f'; // Dark red for incorrect
        }
    });
}

function updateProgressBar() {
    const progress = ((currentQuestionIndex + 1) / totalQuestions) * 100;
    document.getElementById('progress-bar').style.width = `${progress}%`;
}

// Ensure the questions load when the page is ready
document.addEventListener('DOMContentLoaded', function () {
    loadQuestion();
});
