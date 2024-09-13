window.checkAnswer = function(questionNumber, correctAnswer) {
    const userAnswer = document.getElementById(`answer${questionNumber}`).value.trim().toLowerCase();
    const feedbackElement = document.getElementById(`feedback${questionNumber}`);

    if (userAnswer === correctAnswer) {
        feedbackElement.innerHTML = `<span style="color: green;">Correct! The answer is <strong>${correctAnswer}</strong>.</span>`;
    } else {
        feedbackElement.innerHTML = `<span style="color: red;">Incorrect. The correct answer is <strong>${correctAnswer}</strong>.</span>`;
    }
}
