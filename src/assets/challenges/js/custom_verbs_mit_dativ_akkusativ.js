function showTip(questionNumber) {
    const tip = document.getElementById(`tip${questionNumber}`);
    tip.classList.toggle('hidden');
}

function checkAnswers() {
    let correctAnswers = [
        ["meinem Freund", "das Buch"],
        ["dem Lehrer", "den Stift"],
        ["den Kindern", "unsere neue Wohnung"]
    ];

    let userAnswers = [
        [document.getElementById('ans1').value.trim(), document.getElementById('ans2').value.trim()],
        [document.getElementById('ans3').value.trim(), document.getElementById('ans4').value.trim()],
        [document.getElementById('ans5').value.trim(), document.getElementById('ans6').value.trim()]
    ];

    let resultText = "";
    let allCorrect = true;

    for (let i = 0; i < correctAnswers.length; i++) {
        let correctDative = correctAnswers[i][0];
        let correctAccusative = correctAnswers[i][1];

        if (userAnswers[i][0] === correctDative && userAnswers[i][1] === correctAccusative) {
            resultText += `<p>Frage ${i + 1}: Richtig!</p>`;
        } else {
            resultText += `<p>Frage ${i + 1}: Falsch! Richtig wäre: ${correctDative}, ${correctAccusative}.</p>`;
            allCorrect = false;
        }
    }

    if (allCorrect) {
        resultText = "<p>Alle Antworten sind korrekt! Gut gemacht!</p>";
    }

    document.getElementById('result').innerHTML = resultText;
}
