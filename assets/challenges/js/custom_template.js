/**
 * CustomTemplateChallenge class extends the base KonnektorenChallenge
 * This is a template class that can be used as a starting point for creating new challenges
 */
class CustomTemplateChallenge extends KonnektorenChallenge {
  /**
   * Constructor for the CustomTemplateChallenge
   * @param {Object} config - Configuration object containing id and data
   */
  constructor(config) {
    super({
      id: config.id,
      type: "custom_template", // Unique identifier for this challenge type
      data: config.data,
    });

    // Initialize DOM elements used in this challenge
    this.elements = {
      title: document.querySelector(".custom-template-challenge__title"),
      questionsContainer: document.querySelector(
        ".custom-template-challenge__questions",
      ),
      checkButton: document.querySelector(
        ".custom-template-challenge__button--check",
      ),
      resetButton: document.querySelector(
        ".custom-template-challenge__button--reset",
      ),
      feedback: document.querySelector(".custom-template-challenge__feedback"),
      results: document.querySelector(".custom-template-challenge__results"),
    };
  }

  /**
   * Translates static text elements on the page
   * Uses the konnektoren translation system
   */
  translateStaticText() {
    this.elements.title.textContent = window.konnektoren.tr(
      "Custom Template Challenge",
    );
    this.elements.checkButton.textContent =
      window.konnektoren.tr("Check Answer");
    this.elements.resetButton.textContent = window.konnektoren.tr("Try Again");
  }

  /**
   * Loads and displays the current question
   * Override this method to customize how questions are displayed
   */
  loadQuestion() {
    const questions = Array.from(this.data.get("questions"));
    this.elements.questionsContainer.innerHTML = "";

    questions.forEach((question, index) => {
      const questionElement = document.createElement("div");
      questionElement.className = "custom-template-challenge__question";

      // Customize this section based on your question format
      const questionText = question.get("question");
      const options = question.get("options");

      questionElement.innerHTML = `
        <p class="custom-template-challenge__question-text">${questionText}</p>
        <div class="custom-template-challenge__options">
          ${this.createOptions(options, index)}
        </div>
        <div class="custom-template-challenge__feedback" id="feedback-${index}"></div>
      `;

      this.elements.questionsContainer.appendChild(questionElement);
    });
  }

  /**
   * Creates option elements for a question
   * @param {Array} options - Array of option strings
   * @param {number} questionIndex - Index of the current question
   * @returns {string} HTML string for options
   */
  createOptions(options, questionIndex) {
    return options
      .map(
        (option, index) => `
        <button
          class="custom-template-challenge__option"
          data-question="${questionIndex}"
          data-option="${index}"
        >
          ${option}
        </button>
      `,
      )
      .join("");
  }

  /**
   * Checks the user's answer and provides feedback
   * @param {string|number} answer - The user's selected answer
   */
  checkAnswer(answer) {
    const questions = Array.from(this.data.get("questions"));
    let allCorrect = true;
    this.state.userAnswers = [];
    this.state.correctAnswers = 0;

    questions.forEach((question, index) => {
      const userAnswer = answer;
      const correctAnswer = question.get("correct");
      const isCorrect = userAnswer === correctAnswer;

      if (!isCorrect) allCorrect = false;

      if (isCorrect) {
        this.state.correctAnswers++;
      }

      this.state.userAnswers.push({
        questionId: question.get("id"),
        userAnswer: userAnswer,
        correctAnswer: correctAnswer,
        isCorrect: isCorrect,
      });

      // Update feedback display
      const feedback = document.getElementById(`feedback-${index}`);
      feedback.textContent = isCorrect
        ? window.konnektoren.tr("Correct!")
        : window.konnektoren.tr("Incorrect!");
      feedback.className = `custom-template-challenge__feedback custom-template-challenge__feedback--${isCorrect ? "correct" : "incorrect"}`;
    });

    if (allCorrect) {
      this.finish();
    }
  }

  /**
   * Sets up event listeners for the challenge
   */
  setupEventListeners() {
    // Listen for option clicks
    this.elements.questionsContainer.addEventListener("click", (event) => {
      const option = event.target.closest(".custom-template-challenge__option");
      if (option) {
        const questionIndex = option.dataset.question;
        const optionIndex = option.dataset.option;
        this.checkAnswer(optionIndex);
      }
    });

    // Listen for button clicks
    this.elements.checkButton?.addEventListener("click", () =>
      this.checkAnswer(),
    );
    this.elements.resetButton?.addEventListener("click", () =>
      this.loadQuestion(),
    );
  }

  /**
   * Displays the results of the challenge
   * Override this method to customize the results display
   */
  displayResults() {
    // Implementation for displaying results
    // This is called when viewing the results page
  }
}

// Initialize the challenge
function initializeChallenge() {
  const challenge = new CustomTemplateChallenge({
    id: "custom_template",
    data: window.konnektoren.challenge.data,
  });

  if (document.querySelector(".custom-template-challenge__results")) {
    challenge.displayResults(window.konnektoren.result);
  } else {
    challenge.initialize();
  }
}

// Start the challenge
initializeChallenge();
