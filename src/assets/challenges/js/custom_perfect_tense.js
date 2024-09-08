document.getElementById("check-btn-ich").addEventListener("click", function () {
    checkAnswers("ich", { q1: "sein", q2: "haben" });
});
document.getElementById("check-btn-du").addEventListener("click", function () {
    checkAnswers("du", { q1: "sein", q2: "haben" });
});
document.getElementById("check-btn-er").addEventListener("click", function () {
    checkAnswers("er", { q1: "sein", q2: "haben" });
});
document.getElementById("check-btn-wir").addEventListener("click", function () {
    checkAnswers("wir", { q1: "sein", q2: "haben" });
});
document.getElementById("check-btn-sie").addEventListener("click", function () {
    checkAnswers("sie", { q1: "haben", q2: "sein" });
});

function checkAnswers(pronoun, correctAnswers) {
    let totalCorrect = 0;
    let totalQuestions = Object.keys(correctAnswers).length;

    for (let i = 1; i <= totalQuestions; i++) {
        const userAnswer = document.getElementById(`q${i}-${pronoun}`).value;
        const correctAnswer = correctAnswers[`q${i}`];
        const resultSpan = document.getElementById(`correct-q${i}-${pronoun}`);

        if (userAnswer === correctAnswer) {
            resultSpan.textContent = "Correct!";
            resultSpan.classList.add("correct");
            resultSpan.classList.remove("wrong");
            totalCorrect++;
        } else {
            resultSpan.textContent = `Wrong! Correct answer: ${correctAnswer}`;
            resultSpan.classList.add("wrong");
            resultSpan.classList.remove("correct");
        }
    }

    document.getElementById("result").textContent = `You got ${totalCorrect} out of ${totalQuestions} correct for "${pronoun}".`;
}
