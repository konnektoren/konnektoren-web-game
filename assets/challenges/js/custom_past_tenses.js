class PastTensesChallenge extends KonnektorenChallenge {
  constructor(config) {
    super({
      id: config.id,
      type: "past_tenses",
      data: config.data,
    });

    this.elements = {
      title: document.querySelector(".past-tenses-challenge__title"),
      questionsContainer: document.querySelector(
        ".past-tenses-challenge__questions",
      ),
      checkButton: document.querySelector(
        ".past-tenses-challenge__button--check",
      ),
      resetButton: document.querySelector(
        ".past-tenses-challenge__button--reset",
      ),
      feedback: document.querySelector(".past-tenses-challenge__feedback"),
      results: document.querySelector(".past-tenses-challenge__results"),
    };
  }

  translateStaticText() {
    this.elements.title.textContent = window.konnektoren.tr(
      "German Past Tenses Exercise",
    );
    this.elements.checkButton.textContent = window.konnektoren.tr("Check");
    this.elements.resetButton.textContent = window.konnektoren.tr("Try Again");
  }

  loadQuestion() {
    const questions = Array.from(this.data.get("questions"));
    this.elements.questionsContainer.innerHTML = "";

    questions.forEach((question, index) => {
      const questionElement = document.createElement("div");
      questionElement.className = "past-tenses-challenge__question";

      const sentence = question.get("sentence");
      const tense = question.get("tense");
      const dropdowns = question.get("dropdowns");

      let htmlContent = `
        <p class="past-tenses-challenge__sentence">
          <span class="past-tenses-challenge__tense">(${tense})</span>
          ${this.createSentenceWithDropdowns(sentence, dropdowns, index)}
        </p>
        <div class="past-tenses-challenge__feedback" id="feedback-${index}"></div>
      `;

      questionElement.innerHTML = htmlContent;
      this.elements.questionsContainer.appendChild(questionElement);
    });
  }

  createSentenceWithDropdowns(sentence, dropdowns, questionIndex) {
    let parts = sentence.split("__");
    let result = parts[0];

    dropdowns.forEach((dropdown, dropdownIndex) => {
      const selectId = `question-${questionIndex}-${dropdownIndex}`;
      const options = dropdown
        .get("options")
        .map((option) => `<option value="${option}">${option}</option>`)
        .join("");

      result += `<select id="${selectId}" class="past-tenses-challenge__select">
                  <option value="">---</option>
                  ${options}
                </select>`;

      if (parts[dropdownIndex + 1]) {
        result += parts[dropdownIndex + 1];
      }
    });

    return result;
  }

  checkAnswer() {
    const questions = Array.from(this.data.get("questions"));
    let allCorrect = true;
    this.state.userAnswers = [];
    this.state.correctAnswers = 0;

    questions.forEach((question, questionIndex) => {
      const dropdowns = question.get("dropdowns");
      let questionCorrect = true;
      let userAnswers = [];

      dropdowns.forEach((dropdown, dropdownIndex) => {
        const select = document.getElementById(
          `question-${questionIndex}-${dropdownIndex}`,
        );
        const userAnswer = select.value;
        const correctAnswer = dropdown.get("correct_answer");

        userAnswers.push(userAnswer);
        if (userAnswer !== correctAnswer) {
          questionCorrect = false;
        }
      });

      const feedback = document.getElementById(`feedback-${questionIndex}`);

      if (questionCorrect) {
        this.state.correctAnswers++;
        feedback.textContent = window.konnektoren.tr("Correct!");
        feedback.className =
          "past-tenses-challenge__feedback past-tenses-challenge__feedback--correct";
      } else {
        allCorrect = false;
        feedback.textContent = `${window.konnektoren.tr("Incorrect!")} ${question.get("explanation")}`;
        feedback.className =
          "past-tenses-challenge__feedback past-tenses-challenge__feedback--incorrect";
      }

      this.state.userAnswers.push({
        questionId: question.get("id"),
        userAnswers: userAnswers,
        isCorrect: questionCorrect,
      });
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
  const challenge = new PastTensesChallenge({
    id: "custom_past_tenses",
    data: window.konnektoren.challenge.data,
  });

  if (document.querySelector(".past-tenses-challenge__results")) {
    challenge.displayResults(window.konnektoren.result);
  } else {
    challenge.initialize();
  }
}

// Start the challenge
initializeChallenge();
