// Access the data from window.konnektoren.challenge.data or use default data for testing
const data = window.konnektoren && window.konnektoren.challenge && window.konnektoren.challenge.data
    ? window.konnektoren.challenge.data
    : new Map([
        ["main_clauses", [
            new Map([["id", "main-1"], ["text", "Ich gehe nach Hause,"]]),
            new Map([["id", "main-2"], ["text", "Es regnet,"]]),
            new Map([["id", "main-3"], ["text", "Er bleibt zu Hause,"]])
        ]],
        ["sub_clauses", [
            new Map([["id", "sub-1"], ["text", "weil es schon spät ist."]]),
            new Map([["id", "sub-2"], ["text", "weil er krank ist."]]),
            new Map([["id", "sub-3"], ["text", "obwohl die Sonne scheint."]])
        ]],
        ["correct_pairs", new Map([
            ["main-1", "sub-1"],
            ["main-2", "sub-3"],
            ["main-3", "sub-2"]
        ])]
    ]);

// Function to generate clauses dynamically
function generateClauses() {
    const mainClausesContainer = document.getElementById('main-clauses');
    const subClausesContainer = document.getElementById('sub-clauses');

    const mainClausesArray = data.get("main_clauses");
    const subClausesArray = data.get("sub_clauses");

    // Generate main clauses
    mainClausesArray.forEach(mainClause => {
        const id = mainClause.get("id");
        const text = mainClause.get("text");

        const li = document.createElement('li');
        li.classList.add('droppable');
        li.id = id;
        li.textContent = text;

        mainClausesContainer.appendChild(li);
    });

    // Generate subordinate clauses
    subClausesArray.forEach(subClause => {
        const id = subClause.get("id");
        const text = subClause.get("text");

        const li = document.createElement('li');
        li.classList.add('draggable');
        li.setAttribute('draggable', 'true');
        li.id = id;
        li.textContent = text;

        subClausesContainer.appendChild(li);
    });
}

// Call the function to generate clauses
generateClauses();

// Implement drag-and-drop functionality
let draggedItem = null;

// Event delegation for drag events on sub-clauses
document.addEventListener('dragstart', function (e) {
    if (e.target && e.target.classList.contains('draggable')) {
        draggedItem = e.target;
        e.dataTransfer.setData('text/plain', e.target.id);
        setTimeout(() => e.target.classList.add('dragging'), 0);
    }
});

document.addEventListener('dragend', function (e) {
    if (e.target && e.target.classList.contains('draggable')) {
        e.target.classList.remove('dragging');
        draggedItem = null;
    }
});

// Event delegation for drop events on main clauses
document.addEventListener('dragover', function (e) {
    if (e.target && e.target.classList.contains('droppable')) {
        e.preventDefault();
    }
});

document.addEventListener('dragenter', function (e) {
    if (e.target && e.target.classList.contains('droppable')) {
        e.preventDefault();
        e.target.classList.add('hovering');
    }
});

document.addEventListener('dragleave', function (e) {
    if (e.target && e.target.classList.contains('droppable')) {
        e.target.classList.remove('hovering');
    }
});

document.addEventListener('drop', function (e) {
    if (e.target && e.target.classList.contains('droppable')) {
        e.preventDefault();
        e.target.classList.remove('hovering');

        const droppedItemId = e.dataTransfer.getData('text/plain');
        const draggedElement = document.getElementById(droppedItemId);

        // Remove existing draggable from droppable if any
        const existingDraggable = e.target.querySelector('.draggable');
        if (existingDraggable) {
            document.getElementById('sub-clauses').appendChild(existingDraggable);
        }

        // Append the dragged element to the droppable
        e.target.appendChild(draggedElement);
    }
});

// Get correct pairs
const correctPairs = data.get("correct_pairs");

// Check the matching
const checkBtn = document.getElementById('check-btn');
const resetBtn = document.getElementById('reset-btn');
const result = document.getElementById('result');

checkBtn.addEventListener('click', function () {
    let isAllCorrect = true;

    const mainClausesArray = data.get("main_clauses");

    mainClausesArray.forEach(mainClause => {
        const mainId = mainClause.get("id");
        const mainElement = document.getElementById(mainId);
        const subElement = mainElement.querySelector('.draggable');

        mainElement.classList.remove('correct', 'incorrect');

        if (subElement) {
            const correctSubId = correctPairs.get(mainId);
            if (subElement.id === correctSubId) {
                mainElement.classList.add('correct');
            } else {
                mainElement.classList.add('incorrect');
                isAllCorrect = false;
            }
        } else {
            mainElement.classList.add('incorrect');
            isAllCorrect = false;
        }
    });

    if (isAllCorrect) {
        result.textContent = 'Richtig!';
        result.style.color = 'green';
        finishChallenge();
    } else {
        result.textContent = 'Es gibt Fehler. Bitte überprüfen Sie Ihre Antworten.';
        result.style.color = 'red';
        resetBtn.style.display = 'inline-block';
    }

    checkBtn.disabled = true;
});

// Reset the game
resetBtn.addEventListener('click', function () {
    const mainClausesArray = data.get("main_clauses");

    mainClausesArray.forEach(mainClause => {
        const mainId = mainClause.get("id");
        const mainElement = document.getElementById(mainId);

        // Move sub-clauses back to the original list
        const subClause = mainElement.querySelector('.draggable');
        if (subClause) {
            document.getElementById('sub-clauses').appendChild(subClause);
        }

        // Remove classes
        mainElement.classList.remove('correct', 'incorrect');
    });

    result.textContent = '';
    resetBtn.style.display = 'none';
    checkBtn.disabled = false;
});

// Finish function
function finishChallenge() {
    const mainClausesArray = data.get("main_clauses");
    let correctCount = 0;
    const total = mainClausesArray.length;

    const userAnswers = [];

    mainClausesArray.forEach(mainClause => {
        const mainId = mainClause.get("id");
        const mainText = mainClause.get("text");
        const mainElement = document.getElementById(mainId);
        const subElement = mainElement.querySelector('.draggable');

        const correctSubId = correctPairs.get(mainId);
        const correctSubClause = data.get("sub_clauses").find(subClause => subClause.get("id") === correctSubId).get("text");

        let userSubText = '';
        let isCorrect = false;

        if (subElement) {
            userSubText = subElement.textContent;
            if (subElement.id === correctSubId) {
                isCorrect = true;
                correctCount++;
            }
        }

        userAnswers.push({
            mainClause: mainText,
            userSubClause: userSubText || 'Keine Auswahl',
            correctSubClause: correctSubClause,
            isCorrect: isCorrect
        });
    });

    const performance = correctCount / total;

    // Prepare result data
    const resultData = {
        answers: userAnswers,
        performance: performance
    };

    // Send the result event if possible
    if (window.konnektoren && window.konnektoren.sendEvent) {
        const event = {
            type: 'Finish',
            result: {
                id: window.konnektoren.challenge.id,
                performance: performance,
                data: resultData
            }
        };
        window.konnektoren.sendEvent(event);
    } else {
        // For testing purposes
        alert(`Ihre Leistung: ${(performance * 100).toFixed(2)}%`);
    }
}
