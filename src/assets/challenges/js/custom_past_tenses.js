document.getElementById('quiz-form').addEventListener('submit', function(event) {
    event.preventDefault();


    const correctAnswers = {
        'q1-1': 'hatte',
        'q1-2': 'gelernt',
        'q2-1': 'ging',
        'q3-1': 'haben',
        'q3-2': 'gehabt',
        'q4-1': 'hatte gemacht'
    };


    const explanations = {
        'q1': "Correct answer: 'Ich hatte Deutsch gelernt, bevor ich nach Deutschland kam.' Explanation: We use Past Perfect (hatte + gelernt) to indicate that learning German happened before coming to Germany.",
        'q2': "Correct answer: 'Gestern ging ich ins Kino.' Explanation: The Preterite form 'ging' is used to describe a completed action in the past.",
        'q3': "Correct answer: 'Letzte Woche haben wir eine tolle Zeit in Berlin gehabt.' Explanation: The Perfect tense (haben + gehabt) is used to indicate a past action with relevance to the present.",
        'q4': "Correct answer: 'Nachdem er seine Hausaufgaben hatte gemacht, ging er schlafen.' Explanation: The Past Perfect (hatte + gemacht) is used because the action of doing homework happened before going to sleep."
    };

    let score = 0;
    let totalQuestions = Object.keys(correctAnswers).length / 2;


    document.querySelectorAll('.explanation').forEach(el => el.textContent = '');


    for (let i = 1; i <= totalQuestions; i++) {
        const answer1 = document.querySelector(`select[name="q${i}-1"]`).value;
        const answer2 = document.querySelector(`select[name="q${i}-2"]`);
        const explanationDiv = document.getElementById(`explanation-q${i}`);

        if (answer1 === correctAnswers[`q${i}-1`] && (!answer2 || answer2.value === correctAnswers[`q${i}-2`])) {
            score++;
        } else {
            explanationDiv.innerHTML = explanations[`q${i}`];
        }
    }

    const results = document.getElementById('results');
    results.innerHTML = `You got ${score} out of ${totalQuestions} questions correct.`;
});
