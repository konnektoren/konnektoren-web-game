document.getElementById('quiz-form').addEventListener('submit', function(event) {
    event.preventDefault();


    const correctAnswers = {
        'q1': 'auf',
        'q2': 'für',
        'q3': 'über',
        'q4': 'auf'
    };


    const explanations = {
        'q1': "Incorrect answer! The correct preposition is 'auf'. The verb 'warten' is always followed by 'auf' when referring to waiting for something or someone.",
        'q2': "Correct answer: 'Er interessiert sich für Kunst.' The verb 'sich interessieren' requires the preposition 'für' when indicating what someone is interested in.",
        'q3': "Correct answer: 'Wir sprechen über das Problem.' The verb 'sprechen' uses 'über' when talking about a topic or issue.",
        'q4': "Correct answer: 'Sie freut sich auf den Urlaub.' 'Sich freuen' takes 'auf' when referring to looking forward to something in the future."
    };

    let score = 0;
    let totalQuestions = Object.keys(correctAnswers).length;


    document.querySelectorAll('.explanation').forEach(el => el.textContent = '');


    for (let i = 1; i <= totalQuestions; i++) {
        const answer = document.querySelector(`select[name="q${i}"]`).value;
        const explanationDiv = document.getElementById(`explanation-q${i}`);

        if (answer === correctAnswers[`q${i}`]) {
            score++;
        } else {
            explanationDiv.innerHTML = explanations[`q${i}`];
        }
    }


    const results = document.getElementById('results');
    results.innerHTML = `You got ${score} out of ${totalQuestions} questions correct.`;
});
