let interval;

function startStep(stepNumber, time) {
    console.log(`Starting step ${stepNumber} with ${time} minutes.`);
    clearInterval(interval);

    // Hide all steps
    document.querySelectorAll('.step').forEach(step => step.style.display = 'none');

    // Show the current step
    const currentStep = document.getElementById(`step-${stepNumber}`);
    if (currentStep) {
        currentStep.style.display = 'block';
        console.log(`Displayed step ${stepNumber}.`);
    } else {
        console.error(`Step ${stepNumber} not found.`);
        return;
    }

    // Display topic and task in Step 2
    if (stepNumber === 2) {
        const selectedTopic = document.getElementById("topic-selector").value;
        const topicText = selectedTopic === "groupwork"
            ? "Gruppenarbeit kostet doch nur Zeit, weil man alles ausdiskutieren muss."
            : "Teamarbeit bietet dem Einzelnen viel mehr Möglichkeiten.";
        document.getElementById("selected-topic").textContent = `Gewähltes Thema: ${topicText}`;
    }

    // Start the timer
    const timerElement = document.getElementById(`timer-${stepNumber}`);
    if (!timerElement) {
        console.error(`Timer element for step ${stepNumber} not found.`);
        return;
    }

    let seconds = time * 60;
    console.log(`Timer for step ${stepNumber} started with ${seconds} seconds.`);

    interval = setInterval(() => {
        const minutes = Math.floor(seconds / 60);
        const remainingSeconds = seconds % 60;
        timerElement.textContent = `${minutes}:${remainingSeconds < 10 ? '0' : ''}${remainingSeconds}`;

        if (seconds > 0) {
            seconds--;
        } else {
            clearInterval(interval);
            console.warn(`Time for step ${stepNumber} is up.`);
            alert('Zeit abgelaufen!');
        }
    }, 1000);
}

function addArgumentRow() {
    const tableBody = document.querySelector("#argument-table tbody");
    const newRow = document.createElement("tr");

    newRow.innerHTML = `
        <td><textarea placeholder="Pro Argument"></textarea></td>
        <td><textarea placeholder="Kontra Argument"></textarea></td>
    `;

    tableBody.appendChild(newRow);
    console.log("Neue Zeile für Argumente hinzugefügt.");
}

// Initialize with Step 1
startStep(1, 5);
