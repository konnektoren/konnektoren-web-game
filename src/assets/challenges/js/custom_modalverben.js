const loesung1 = ["will", "musst", "möchte", "soll", "dürfen", "müsst", "will"];
const erklaerung1 = [
    "Use 'will' to express a future intention.",
    "Use 'musst' to express necessity.",
    "Use 'möchte' to express a wish or desire.",
    "Use 'soll' to express an obligation or recommendation.",
    "Use 'dürfen' to express permission.",
    "Use 'müsst' to express prohibition.",
    "Use 'will' to express a future intention."
];

const falscheAntwortErklaerung1 = [
    "Only 'will' works here to show intention; other modal verbs like 'muss' or 'möchte' don't express this.",
    "'Musst' is the correct modal verb for expressing necessity, not 'kann' or 'möchte'.",
    "'Möchte' expresses a wish, whereas other modal verbs like 'muss' express necessity, which doesn't fit here.",
    "'Soll' indicates a recommendation or obligation, while 'müssen' or 'dürfen' don't carry this meaning here.",
    "'Dürfen' means permission, which fits, while other modal verbs (like 'können') don't imply permission in this context.",
    "'Müsst' indicates prohibition, fitting the negative instruction, unlike 'dürfen', which means permission.",
    "'Will' expresses a strong intention, while 'möchte' expresses a more polite or softer desire."
];

function checkAufgabe1() {
    let correct = true;
    let resultMessages = [];

    for (let i = 1; i <= 7; i++) {
        const answer = document.getElementById(`aufgabe1_${i}`).value;
        const feedbackElement = document.getElementById(`feedback1_${i}`);

        if (answer !== loesung1[i - 1]) {
            correct = false;
            document.getElementById(`aufgabe1_${i}`).style.border = "2px solid red";
            feedbackElement.innerHTML = `Falsche Antwort. Die richtige Antwort ist: '${loesung1[i - 1]}'.<br> 
            Erklärung: ${erklaerung1[i - 1]}<br> 
            Warum andere Antworten falsch sind: ${falscheAntwortErklaerung1[i - 1]}`;
            feedbackElement.style.color = "red";
            resultMessages.push(`Frage ${i}: Falsch`);
        } else {
            document.getElementById(`aufgabe1_${i}`).style.border = "2px solid green";
            feedbackElement.innerHTML = "Richtige Antwort!";
            feedbackElement.style.color = "green";
            resultMessages.push(`Frage ${i}: Richtig`);
        }
    }

    document.getElementById("result1").innerText = correct ? "Alles richtig!" : "Leider sind einige Antworten falsch.";


    alert(resultMessages.join('\n'));
}

function resetAufgabe1() {
    for (let i = 1; i <= 7; i++) {
        document.getElementById(`aufgabe1_${i}`).selectedIndex = 0; // Reset Dropdown
        document.getElementById(`feedback1_${i}`).innerHTML = ""; // Clear feedback
        document.getElementById(`aufgabe1_${i}`).style.border = ""; // Reset border style
    }
    document.getElementById("result1").innerText = ""; // Clear result
}
