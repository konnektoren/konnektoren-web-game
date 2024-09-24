document.getElementById('exercise-form').addEventListener('submit', function(event) {
    event.preventDefault();


    const correctAnswers = {
        'q1': 'kann',
        'q2': 'musst',
        'q3': 'braucht zu',
        'q4': 'sollen'
    };


    const explanations = {
        'q1': {
            correct: "Correct answer: 'kann'. 'Können' is a modal verb used to express ability or possibility.",
            wrong: "Incorrect! You should use the modal verb 'kann' to express ability here."
        },
        'q2': {
            correct: "Correct answer: 'musst'. 'Müssen' is a modal verb used to express necessity or obligation.",
            wrong: "Incorrect! You should use the modal verb 'musst' to express necessity or obligation."
        },
        'q3': {
            correct: "Correct answer: 'braucht zu'. 'Brauchen + zu' is used to express the lack of necessity.",
            wrong: "Incorrect! The construction 'brauchen + zu' is required here to indicate that there is no need to tidy up."
        },
        'q4': {
            correct: "Correct answer: 'sollen'. 'Sollen' is a modal verb used to express advice or suggestion.",
            wrong: "Incorrect! The modal verb 'sollen' should be used to give advice or suggest an action."
        }
    };

    let score = 0;
    let totalQuestions = Object.keys(correctAnswers).length;


    document.querySelectorAll('.explanation').forEach(el => el.textContent = '');


    for (let i = 1; i <= totalQuestions; i++) {
        const answer = document.querySelector(`input[name="q${i}"]`).value.trim();
        const explanationDiv = document.getElementById(`explanation-q${i}`);

        if (answer === correctAnswers[`q${i}`]) {
            score++;
            explanationDiv.innerHTML = explanations[`q${i}`].correct;
        } else {
            explanationDiv.innerHTML = explanations[`q${i}`].wrong;
        }
    }


    const results = document.getElementById('results');
    results.innerHTML = `You got ${score} out of ${totalQuestions} questions correct.`;
});
