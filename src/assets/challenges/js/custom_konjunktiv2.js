let currentQuestionIndex = 0;
let score = 0;

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

    const verb1Select = document.getElementById('verb1');
    const verb2Select = document.getElementById('verb2');

    // Clear the current options and reset select dropdowns
    verb1Select.innerHTML = '<option value="">Select</option>';
    verb2Select.innerHTML = '<option value="">Select</option>';

    // Shuffle and add verb1 options
    const shuffledVerb1 = shuffleOptions([...question.verb1]);
    shuffledVerb1.forEach(option => {
        verb1Select.innerHTML += `<option value="${option}">${option}</option>`;
    });

    // Shuffle and add verb2 options
    const shuffledVerb2 = shuffleOptions([...question.verb2]);
    shuffledVerb2.forEach(option => {
        verb2Select.innerHTML += `<option value="${option}">${option}</option>`;
    });

    document.getElementById('hint-box').textContent = `💡 ${question.hint}`;
    document.getElementById('hint-box').classList.remove('hidden');

    // Reset feedback and styles
    document.getElementById('feedback').textContent = '';
    verb1Select.style.backgroundColor = '';
    verb2Select.style.backgroundColor = '';

    // Add event listeners to dropdowns
    verb1Select.addEventListener('change', checkBothSelected);
    verb2Select.addEventListener('change', checkBothSelected);
}

function checkBothSelected() {
    const verb1 = document.getElementById('verb1').value;
    const verb2 = document.getElementById('verb2').value;

    // Only check answer if both dropdowns have a value selected
    if (verb1 !== '' && verb2 !== '') {
        checkAnswer();
    }
}

function checkAnswer() {
    const verb1 = document.getElementById('verb1').value;
    const verb2 = document.getElementById('verb2').value;
    const question = questions[currentQuestionIndex];

    const correctVerb1 = question.correctAnswers.verb1;
    const correctVerb2 = question.correctAnswers.verb2;

    let feedback = '';
    let feedbackColor = '';

    if (verb1 === correctVerb1 && verb2 === correctVerb2) {
        feedback = 'Correct! Well done.';
        feedbackColor = '#388e3c'; // Dark green
        score += 10;

        // Move to the next question after a short delay
        setTimeout(() => {
            currentQuestionIndex = (currentQuestionIndex + 1) % questions.length;
            loadQuestion();
        }, 2000); // 2-second delay to allow reviewing the correct answer
    } else {
        // Show the correct answer and why it's correct
        feedback = `Try again! The correct answer is "${correctVerb1}" and "${correctVerb2}". ${question.hint}`;
        feedbackColor = '#d32f2f'; // Dark red
    }

    // Set feedback message
    document.getElementById('feedback').textContent = feedback;
    document.getElementById('feedback').style.color = feedbackColor;
    document.getElementById('score').textContent = `Score: ${score}`;

    // Highlight correct answers and incorrect choices
    const verb1Select = document.getElementById('verb1');
    const verb2Select = document.getElementById('verb2');

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
}
