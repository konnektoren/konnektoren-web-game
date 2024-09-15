// Access the data from window.konnektoren.challenge.data or use default data for testing
const data = window.konnektoren && window.konnektoren.challenge && window.konnektoren.challenge.data
    ? window.konnektoren.challenge.data
    : new Map([
        ["questions", [
            new Map([
                ["scenario", 'Wenn ich ______ (sein) reich, ______ (reisen) um die Welt.'],
                ["verb1", ['wäre', 'bin', 'war']],
                ["verb2", ['würde reisen', 'reiste', 'reise']],
                ["correctAnswers", new Map([["verb1", 'wäre'], ["verb2", 'würde reisen']])],
                ["hint", 'Think about what you would "be" or "do" if you were rich.']
            ]),
            new Map([
                ["scenario", 'Wenn er mehr Zeit ______ (haben), ______ (arbeiten) an seinem Projekt.'],
                ["verb1", ['hätte', 'hat', 'hatte']],
                ["verb2", ['würde arbeiten', 'arbeitete', 'arbeitet']],
                ["correctAnswers", new Map([["verb1", 'hätte'], ["verb2", 'würde arbeiten']])],
                ["hint", 'Think about what he would "have" or "do" if he had more time.']
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
    const verb1Options = shuffleOptions([...question.get("verb1")]);
    const verb2Options = shuffleOptions([...question.get("verb2")]);
    const hint = question.get("hint");

    // Update scenario text
    document.getElementById('scenario-text').textContent = scenarioText;

    // Populate the first verb dropdown
    const verb1Select = document.getElementById('verb1');
    verb1Select.innerHTML = '<option value="">Select</option>'; // Reset options

    verb1Options.forEach(option => {
        const opt = document.createElement('option');
        opt.value = option;
        opt.textContent = option;
        verb1Select.appendChild(opt);
    });

    // Populate the second verb dropdown
    const verb2Select = document.getElementById('verb2');
    verb2Select.innerHTML = '<option value="">Select</option>'; // Reset options

    verb2Options.forEach(option => {
        const opt = document.createElement('option');
        opt.value = option;
        opt.textContent = option;
        verb2Select.appendChild(opt);
    });

    // Update hint
    const hintBox = document.getElementById('hint-box');
    hintBox.textContent = `💡 Hint: ${hint}`;
    hintBox.classList.remove('hidden');

    // Reset feedback and styles
    const feedback = document.getElementById('feedback');
    feedback.textContent = '';
    verb1Select.style.backgroundColor = '';
    verb2Select.style.backgroundColor = '';

    // Enable the select dropdowns
    verb1Select.disabled = false;
    verb2Select.disabled = false;
}

// Function to check both dropdowns are selected before checking the answer
function checkBothSelected() {
    const verb1 = document.getElementById('verb1').value;
    const verb2 = document.getElementById('verb2').value;

    // Only check answer if both dropdowns have a value selected
    if (verb1 !== '' && verb2 !== '') {
        checkAnswer();
    }
}

// Function to check the user's answer
function checkAnswer() {
    const question = questions[currentQuestionIndex];
    const verb1 = document.getElementById('verb1').value;
    const verb2 = document.getElementById('verb2').value;

    const correctVerb1 = question.get("correctAnswers").get("verb1");
    const correctVerb2 = question.get("correctAnswers").get("verb2");

    const feedbackElement = document.getElementById('feedback');
    const verb1Select = document.getElementById('verb1');
    const verb2Select = document.getElementById('verb2');

    let isCorrect = false;
    let feedback = '';
    let feedbackColor = '';

    if (verb1 === correctVerb1 && verb2 === correctVerb2) {
        feedback = 'Correct! Well done.';
        feedbackColor = '#388e3c'; // Dark green
        score += 10;
        isCorrect = true;
    } else {
        feedback = `Try again! The correct answers are "${correctVerb1}" and "${correctVerb2}". ${question.get("hint")}`;
        feedbackColor = '#d32f2f'; // Dark red
    }

    // Set feedback message
    feedbackElement.textContent = feedback;
    feedbackElement.style.color = feedbackColor;
    document.getElementById('score').textContent = `Score: ${score}`;

    // Highlight correct answers
    verb1Select.querySelectorAll('option').forEach(option => {
        option.style.backgroundColor = option.value === correctVerb1 ? '#388e3c' : '';
        if (option.value === verb1 && option.value !== correctVerb1) {
            option.style.backgroundColor = '#d32f2f'; // Dark red for incorrect
        }
    });

    verb2Select.querySelectorAll('option').forEach(option => {
        option.style.backgroundColor = option.value === correctVerb2 ? '#388e3c' : '';
        if (option.value === verb2 && option.value !== correctVerb2) {
            option.style.backgroundColor = '#d32f2f'; // Dark red for incorrect
        }
    });

    // Update progress bar if correct
    if (isCorrect) {
        updateProgressBar();
        // Move to next question after a short delay
        setTimeout(() => {
            currentQuestionIndex++;
            loadQuestion();
        }, 2000); // 2-second delay to allow reviewing the correct answer
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
    document.getElementById('verb1').disabled = true;
    document.getElementById('verb2').disabled = true;

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
        const verb1Select = document.getElementById('verb1');
        const verb2Select = document.getElementById('verb2');
        const selectedVerb1 = verb1Select.options[index + 1]?.value || 'Keine Auswahl'; // Adjusting for 'Select' option
        const selectedVerb2 = verb2Select.options[index + 1]?.value || 'Keine Auswahl'; // Adjusting for 'Select' option
        const correctVerb1 = question.get("correctAnswers").get("verb1");
        const correctVerb2 = question.get("correctAnswers").get("verb2");

        answers.push({
            questionNumber: index + 1,
            scenario: question.get("scenario"),
            selectedVerb1: selectedVerb1,
            selectedVerb2: selectedVerb2,
            correctVerb1: correctVerb1,
            correctVerb2: correctVerb2,
            isVerb1Correct: selectedVerb1 === correctVerb1,
            isVerb2Correct: selectedVerb2 === correctVerb2
        });
    });

    return { answers: answers };
}

// Event listeners for the select dropdowns
document.getElementById('verb1').addEventListener('change', checkBothSelected);
document.getElementById('verb2').addEventListener('change', checkBothSelected);

loadQuestion();
