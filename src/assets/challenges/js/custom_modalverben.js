class ModalVerbenChallenge extends KonnektorenChallenge {
  constructor(config) {
    super({
      id: config.id,
      type: "modalverben",
      data: config.data,
    });

    this.elements = {
      title: document.querySelector(".modal-verbs-challenge__title"),
      instruction: document.querySelector(
        ".modal-verbs-challenge__instruction",
      ),
      questionsContainer: document.querySelector(
        ".modal-verbs-challenge__questions",
      ),
      checkButton: document.querySelector(
        ".modal-verbs-challenge__button--check",
      ),
      resetButton: document.querySelector(
        ".modal-verbs-challenge__button--reset",
      ),
      feedback: document.querySelector(".modal-verbs-challenge__feedback"),
      results: document.querySelector(".modal-verbs-challenge__results"),
    };
  }

  translateStaticText() {
    this.elements.title.textContent = window.konnektoren.tr(
      "Modal Verbs Exercise",
    );
    this.elements.instruction.textContent = window.konnektoren.tr(
      "Fill in the gaps with the correct modal verbs",
    );
    this.elements.checkButton.textContent = window.konnektoren.tr("Check");
    this.elements.resetButton.textContent = window.konnektoren.tr("Try Again");
  }

  loadQuestion() {
    const questions = Array.from(this.data.get("questions"));
    this.elements.questionsContainer.innerHTML = "";

    questions.forEach((question, index) => {
      const questionElement = document.createElement("div");
      questionElement.className = "modal-verbs-challenge__question";

      const select = document.createElement("select");
      select.className = "modal-verbs-challenge__select";
      select.id = `question-${index}`;

      // Add default option
      const defaultOption = document.createElement("option");
      defaultOption.value = "";
      defaultOption.textContent = "Select";
      select.appendChild(defaultOption);

      // Add variants (options) from the YAML structure
      question.get("variants").forEach((variant) => {
        const optionElement = document.createElement("option");
        optionElement.value = variant;
        optionElement.textContent = variant;
        select.appendChild(optionElement);
      });

      // Create question text with sentence and translation
      const sentence = question.get("sentence");
      const translation = question.get("translation");
      const [before, after] = sentence.split("_______");

      questionElement.innerHTML = `
        <div class="modal-verbs-challenge__question-content">
          <p class="modal-verbs-challenge__question-text">
            ${before}${select.outerHTML}${after}
          </p>
          <p class="modal-verbs-challenge__question-translation">
            ${translation}
          </p>
          <div class="modal-verbs-challenge__feedback" id="feedback-${index}"></div>
        </div>
      `;

      this.elements.questionsContainer.appendChild(questionElement);
    });
  }

  checkAnswer() {
    const questions = Array.from(this.data.get("questions"));
    let allCorrect = true;
    this.state.userAnswers = [];
    this.state.correctAnswers = 0;

    questions.forEach((question, index) => {
      const select = document.getElementById(`question-${index}`);
      const feedback = document.getElementById(`feedback-${index}`);
      const userAnswer = select.value;
      const correctAnswer = question.get("correct_answer");

      const isCorrect = userAnswer === correctAnswer;

      if (isCorrect) {
        this.state.correctAnswers++;
      }

      if (!isCorrect) allCorrect = false;

      this.state.userAnswers.push({
        questionId: question.get("id"),
        userAnswer: userAnswer,
        correctAnswer: correctAnswer,
        isCorrect: isCorrect,
      });

      feedback.textContent = isCorrect
        ? window.konnektoren.tr("Correct!")
        : `${window.konnektoren.tr("Incorrect!")} ${question.get("explanation")}`;
      feedback.className = `modal-verbs-challenge__feedback modal-verbs-challenge__feedback--${isCorrect ? "correct" : "incorrect"}`;
    });

    if (allCorrect) {
      this.finish();
    }
  }

  setupEventListeners() {
    this.elements.checkButton.addEventListener("click", () =>
      this.checkAnswer(),
    );
    this.elements.resetButton.addEventListener("click", () =>
      this.loadQuestion(),
    );
  }
}

// Initialize the challenge
function initializeChallenge() {
  const challenge = new ModalVerbenChallenge({
    id: "custom_modalverben",
    data: window.konnektoren.challenge.data,
  });

  if (document.querySelector(".modal-verbs-challenge__results")) {
    challenge.displayResults(window.konnektoren.result);
  } else {
    challenge.initialize();
  }
}

// Start the challenge
initializeChallenge();
