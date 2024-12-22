"use strict";

class WritingTaskChallenge extends KonnektorenChallenge {
  constructor(config) {
    super({
      id: config.id,
      type: "writing_task",
      data: config.data,
    });

    this.currentStep = 1;
    this.timers = new Map();

    this.elements = {
      title: document.querySelector(".writing-task__title"),
      steps: document.querySelectorAll(".writing-task__step"),
      topicSelector: document.querySelector(".writing-task__topic-selector"),
      selectedTopic: document.querySelector(".writing-task__selected-topic"),
      argumentsTable: document.querySelector(".writing-task__arguments tbody"),
      timers: document.querySelectorAll(".writing-task__timer"),
      nextButtons: document.querySelectorAll(".writing-task__button--next"),
      addButton: document.querySelector(".writing-task__button--add"),
    };

    this.initialize();
  }

  initialize() {
    this.initializeElements();
    this.setupEventListeners();
    this.startCurrentStep();
  }

  initializeElements() {
    // Hide all steps except first
    this.elements.steps.forEach((step, index) => {
      step.style.display = index === 0 ? "block" : "none";
    });

    // Initialize topic selector
    const options = this.data.get("options");
    options.forEach((option) => {
      const optEl = document.createElement("option");
      optEl.value = option.get("id");
      optEl.textContent = option.get("label");
      this.elements.topicSelector.appendChild(optEl);
    });
  }

  setupEventListeners() {
    this.elements.nextButtons.forEach((button) => {
      button.addEventListener("click", () => this.nextStep());
    });

    this.elements.addButton?.addEventListener("click", () =>
      this.addArgumentRow(),
    );
  }

  startCurrentStep() {
    const steps = this.data.get("steps");
    const currentStepData = steps[this.currentStep - 1];
    const duration = currentStepData.get("duration");
    const timerElement = this.elements.timers[this.currentStep - 1];

    if (timerElement && duration) {
      this.startTimer(duration, timerElement);
    }
  }

  startTimer(minutes, timerElement) {
    let seconds = minutes * 60;

    // Clear existing timer if any
    if (this.timers.has(this.currentStep)) {
      clearInterval(this.timers.get(this.currentStep));
    }

    const timer = setInterval(() => {
      const mins = Math.floor(seconds / 60);
      const secs = seconds % 60;
      timerElement.textContent = `${mins}:${secs.toString().padStart(2, "0")}`;

      if (--seconds < 0) {
        clearInterval(timer);
        alert(window.konnektoren.tr("Time's up!"));
      }
    }, 1000);

    this.timers.set(this.currentStep, timer);
  }

  nextStep() {
    // Clear current timer
    if (this.timers.has(this.currentStep)) {
      clearInterval(this.timers.get(this.currentStep));
    }

    // Hide current step
    this.elements.steps[this.currentStep - 1].style.display = "none";

    // Show next step
    this.currentStep++;
    if (this.currentStep <= this.elements.steps.length) {
      this.elements.steps[this.currentStep - 1].style.display = "block";
      this.startCurrentStep();
    } else {
      this.finish();
    }
  }

  addArgumentRow() {
    const row = document.createElement("tr");
    row.innerHTML = `
            <td>
                <textarea class="writing-task__argument writing-task__argument--pro"
                          placeholder="Pro Argument"></textarea>
            </td>
            <td>
                <textarea class="writing-task__argument writing-task__argument--contra"
                          placeholder="Kontra Argument"></textarea>
            </td>
        `;
    this.elements.argumentsTable.appendChild(row);
  }

  finish() {
    // Cleanup timers
    this.timers.forEach((timer) => clearInterval(timer));

    // Create result object
    const result = {
      topic: this.elements.topicSelector.value,
      arguments: this.collectArguments(),
      // Add other collected data as needed
    };

    // Send finish event
    window.konnektoren.sendEvent({
      type: "Finish",
      result: result,
    });
  }

  collectArguments() {
    const argumentsList = []; // Changed from 'arguments' to 'argumentsList'
    const rows = this.elements.argumentsTable.querySelectorAll("tr");
    rows.forEach((row) => {
      argumentsList.push({
        pro: row.querySelector(".writing-task__argument--pro").value,
        contra: row.querySelector(".writing-task__argument--contra").value,
      });
    });
    return argumentsList;
  }
}

// Initialize the challenge
function initializeChallenge() {
  const challenge = new WritingTaskChallenge({
    id: "custom_schriftliche_aufgabe",
    data: window.konnektoren.challenge.data,
  });
}

// Start the challenge
initializeChallenge();
