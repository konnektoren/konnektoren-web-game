let draggedItem = null;

// Make Nebensätze draggable
const draggables = document.querySelectorAll('.draggable');
draggables.forEach(draggable => {
    draggable.addEventListener('dragstart', function(e) {
        draggedItem = this;
        e.dataTransfer.setData('text', e.target.id); // Safari fix: Set data to transfer
        setTimeout(() => this.classList.add('dragging'), 0);
    });

    draggable.addEventListener('dragend', function() {
        setTimeout(() => this.classList.remove('dragging'), 0);
        draggedItem = null;
    });
});

// Make Hauptsätze droppable
const droppables = document.querySelectorAll('.droppable');
droppables.forEach(droppable => {
    droppable.addEventListener('dragover', function(e) {
        e.preventDefault(); // Allow drop
        this.classList.add('hovering');

        // Add temporary classes for drag feedback
        const isCorrect = checkIfCorrect(draggedItem, this);
        if (isCorrect) {
            this.classList.add('correct-during-drag');
            this.classList.remove('incorrect-during-drag');
        } else {
            this.classList.add('incorrect-during-drag');
            this.classList.remove('correct-during-drag');
        }
    });

    droppable.addEventListener('dragleave', function() {
        this.classList.remove('hovering', 'correct-during-drag', 'incorrect-during-drag');
    });

    droppable.addEventListener('drop', function(e) {
        e.preventDefault(); // Ensure drop is handled properly
        this.classList.remove('hovering', 'correct-during-drag', 'incorrect-during-drag');

        const droppedItemId = e.dataTransfer.getData('text'); // Safari fix: Get transferred data
        const draggedElement = document.getElementById(droppedItemId);

        // Reset classes for immediate feedback
        droppables.forEach(droppable => {
            droppable.classList.remove('incorrect');
        });

        // Check if the dropped item is correct
        const isCorrect = checkIfCorrect(draggedElement, this);

        if (isCorrect) {
            this.appendChild(draggedElement); // Drop the dragged item into the main clause
            this.classList.add('correct'); // Mark as correct
        } else {
            this.classList.add('incorrect'); // Mark as incorrect
            // Optional: Move the item back if it's incorrect
            setTimeout(() => document.getElementById('sub-clauses').appendChild(draggedElement), 1000);
        }
    });
});

function checkIfCorrect(draggedElement, droppableElement) {
    const correctPairs = {
        'main-1': 'sub-1', // Ich gehe nach Hause, weil es schon spät ist.
        'main-2': 'sub-3', // Es regnet, obwohl die Sonne scheint.
        'main-3': 'sub-2', // Er bleibt zu Hause, weil er krank ist.
    };

    const mainId = droppableElement.id;
    const correctSubId = correctPairs[mainId];

    return draggedElement.id === correctSubId;
}

// Check the matching
const checkBtn = document.getElementById('check-btn');
const resetBtn = document.getElementById('reset-btn');
const result = document.getElementById('result');

checkBtn.addEventListener('click', function() {
    let isCorrect = true;

    Object.keys(correctPairs).forEach(mainId => {
        const mainElement = document.getElementById(mainId);
        const subElement = mainElement.querySelector('.draggable');

        if (!subElement || subElement.id !== correctPairs[mainId]) {
            isCorrect = false;
            mainElement.classList.add('incorrect'); // Mark as incorrect
        } else {
            mainElement.classList.add('correct'); // Mark as correct
        }
    });

    if (isCorrect) {
        result.textContent = 'Richtig!';
        result.style.color = 'green';
    } else {
        result.textContent = 'Falsch, versuchen Sie es erneut.';
        result.style.color = 'red';
        resetBtn.style.display = 'block'; // Show reset button
    }

    checkBtn.disabled = true; // Disable check button after submission
});

// Reset the game
resetBtn.addEventListener('click', function() {
    droppables.forEach(droppable => {
        // Remove Nebensatz (subordinate clause) if it's inside a Hauptsatz (main clause)
        const subClause = droppable.querySelector('.draggable');
        if (subClause) {
            document.getElementById('sub-clauses').appendChild(subClause); // Move back to original list
        }

        // Reset classes
        droppable.classList.remove('correct', 'incorrect', 'correct-during-drag', 'incorrect-during-drag');
    });

    result.textContent = ''; // Clear result message
    resetBtn.style.display = 'none'; // Hide reset button
    checkBtn.disabled = false; // Re-enable check button
});
