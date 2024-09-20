document.getElementById('quiz-form').addEventListener('submit', function(event) {
    event.preventDefault();


    const correctAnswers = {
        'q1': 'wann',
        'q2': 'wo',
        'q3': 'warum',
        'q4': 'wie'
    };


    const explanations = {
        'q1': {
            correct: "Correct answer: 'Wann gehst du zur Schule?' 'Wann' is used to ask about the time something happens.",
            wrong: {
                'wo': "Incorrect! 'Wo' asks for a location, but here we need to ask for the time. The correct particle is 'wann'.",
                'warum': "Incorrect! 'Warum' asks for a reason, but here we need to ask about time. The correct particle is 'wann'."
            }
        },
        'q2': {
            correct: "Correct answer: 'Wo wohnst du?' 'Wo' is used to ask for a location.",
            wrong: {
                'wie': "Incorrect! 'Wie' asks for manner, but here we're asking about location. The correct particle is 'wo'.",
                'wann': "Incorrect! 'Wann' asks for time, but here we're asking for a place. The correct particle is 'wo'."
            }
        },
        'q3': {
            correct: "Correct answer: 'Warum möchtest du dieses Buch lesen?' 'Warum' is used to ask for a reason.",
            wrong: {
                'wie': "Incorrect! 'Wie' asks about manner, but here we need to ask for a reason. The correct particle is 'warum'.",
                'wann': "Incorrect! 'Wann' asks for time, but here we need to ask for a reason. The correct particle is 'warum'."
            }
        },
        'q4': {
            correct: "Correct answer: 'Wie ist dein Name?' 'Wie' is used to ask for a description or form, such as someone's name.",
            wrong: {
                'was': "Incorrect! 'Was' asks for an object or thing, but here we ask about the form of the name. The correct particle is 'wie'.",
                'warum': "Incorrect! 'Warum' asks for a reason, but here we need to ask for the form of the name. The correct particle is 'wie'."
            }
        }
    };

    let score = 0;
    let totalQuestions = Object.keys(correctAnswers).length;

    // Clear previous explanations
    document.querySelectorAll('.explanation').forEach(el => el.textContent = '');


    for (let i = 1; i <= totalQuestions; i++) {
        const answer = document.querySelector(`select[name="q${i}"]`).value;
        const explanationDiv = document.getElementById(`explanation-q${i}`);

        if (answer === correctAnswers[`q${i}`]) {
            score++;
            explanationDiv.innerHTML = explanations[`q${i}`].correct;
        } else {
            explanationDiv.innerHTML = explanations[`q${i}`].wrong[answer];
        }
    }


    const results = document.getElementById('results');
    results.innerHTML = `You got ${score} out of ${totalQuestions} questions correct.`;
});
