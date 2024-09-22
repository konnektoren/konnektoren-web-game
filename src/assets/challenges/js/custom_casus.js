document.getElementById('quiz-form').addEventListener('submit', function(event) {
    event.preventDefault();

    const correctAnswers = {
        q1: 'nominative',
        q2: 'accusative',
        q3: 'genitive',
        q4: 'dative'
    };

    let score = 0;
    let totalQuestions = Object.keys(correctAnswers).length;

    for (let key in correctAnswers) {
        const userAnswer = document.querySelector(`input[name="${key}"]:checked`);
        if (userAnswer && userAnswer.value === correctAnswers[key]) {
            score++;
        }
    }

    const results = document.getElementById('results');
    results.textContent = `Du hast ${score} von ${totalQuestions} Fragen richtig beantwortet.`;
});
