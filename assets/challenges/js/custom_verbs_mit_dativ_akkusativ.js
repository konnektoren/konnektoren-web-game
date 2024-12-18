class VerbsDativAkkusativChallenge extends KonnektorenChallenge {
  constructor(config) {
    super({
      id: config.id,
      type: "verbs_dativ_akkusativ",
      data: config.data,
    });

    this.elements = {
      title: document.querySelector(".verbs-dativ-akkusativ__title"),
      description: document.querySelector(
        ".verbs-dativ-akkusativ__description",
      ),
      questionsContainer: document.querySelector(
        ".verbs-dativ-akkusativ__questions",
      ),
      checkButton: document.querySelector(
        ".verbs-dativ-akkusativ__button--check",
      ),
      resetButton: document.querySelector(
        ".verbs-dativ-akkusativ__button--reset",
      ),
      feedback: document.querySelector(".verbs-dativ-akkusativ__feedback"),
    };
  }

  translateStaticText() {
    this.elements.title.textContent = window.konnektoren.tr(
      "Verbs with Dative and Accusative",
    );
    this.elements.description.textContent = window.konnektoren.tr(
      "Fill in the gaps with the correct dative and accusative forms.",
    );
    this.elements.checkButton.textContent =
      window.konnektoren.tr("Check Answers");
    this.elements.resetButton.textContent = window.konnektoren.tr("Try Again");
  }

  loadQuestion() {
    const questions = Array.from(this.data.get("questions"));
    this.elements.questionsContainer.innerHTML = "";

    questions.forEach((question, index) => {
      const questionElement = document.createElement("div");
      questionElement.className = "verbs-dativ-akkusativ__question";

      questionElement.innerHTML = `
        <p class="verbs-dativ-akkusativ__sentence">${question.get("sentence")}</p>
        <div class="verbs-dativ-akkusativ__inputs">
          <input type="text"
                 class="verbs-dativ-akkusativ__input"
                 id="dative-${index}"
                 placeholder="${window.konnektoren.tr("Dative form")}">
          <input type="text"
                 class="verbs-dativ-akkusativ__input"
                 id="accusative-${index}"
                 placeholder="${window.konnektoren.tr("Accusative form")}">
        </div>
        <button type="button"
                class="verbs-dativ-akkusativ__tip-button"
                onclick="window.showTip(${index})">
          ${window.konnektoren.tr("Show Tip")}
        </button>
        <p class="verbs-dativ-akkusativ__tip hidden" id="tip-${index}">
          ${question.get("tip")}
        </p>
        <div class="verbs-dativ-akkusativ__feedback" id="feedback-${index}"></div>
      `;

      this.elements.questionsContainer.appendChild(questionElement);
    });
  }

  showTip(index) {
    const tipElement = document.getElementById(`tip-${index}`);
    tipElement.classList.toggle("hidden");
  }

  checkAnswer() {
    const questions = Array.from(this.data.get("questions"));
    let allCorrect = true;
    this.state.userAnswers = [];
    this.state.correctAnswers = 0;

    questions.forEach((question, index) => {
      const dativeInput = document.getElementById(`dative-${index}`);
      const accusativeInput = document.getElementById(`accusative-${index}`);
      const feedback = document.getElementById(`feedback-${index}`);

      const userDative = dativeInput.value.trim();
      const userAccusative = accusativeInput.value.trim();
      const correctDative = question.get("correct_answers").get("dative");
      const correctAccusative = question
        .get("correct_answers")
        .get("accusative");

      const isCorrect =
        userDative === correctDative && userAccusative === correctAccusative;

      if (!isCorrect) allCorrect = false;

      if (isCorrect) {
        this.state.correctAnswers++;
        dativeInput.className =
          "verbs-dativ-akkusativ__input verbs-dativ-akkusativ__input--correct";
        accusativeInput.className =
          "verbs-dativ-akkusativ__input verbs-dativ-akkusativ__input--correct";
        feedback.textContent = window.konnektoren.tr("Correct!");
        feedback.className =
          "verbs-dativ-akkusativ__feedback verbs-dativ-akkusativ__feedback--correct";
      } else {
        dativeInput.className =
          "verbs-dativ-akkusativ__input verbs-dativ-akkusativ__input--incorrect";
        accusativeInput.className =
          "verbs-dativ-akkusativ__input verbs-dativ-akkusativ__input--incorrect";
        feedback.innerHTML = `${window.konnektoren.tr("Incorrect!")}<br>${window.konnektoren.tr("Correct answer")}: ${correctDative}, ${correctAccusative}<br>${question.get("explanation")}`;
        feedback.className =
          "verbs-dativ-akkusativ__feedback verbs-dativ-akkusativ__feedback--incorrect";
      }

      this.state.userAnswers.push({
        questionId: question.get("id"),
        userDative: userDative,
        userAccusative: userAccusative,
        correctDative: correctDative,
        correctAccusative: correctAccusative,
        isCorrect: isCorrect,
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
    window.showTip = (index) => this.showTip(index);
  }
}

// Initialize the challenge
function initializeChallenge() {
  const challenge = new VerbsDativAkkusativChallenge({
    id: "custom_verbs_mit_dativ_akkusativ",
    data: window.konnektoren.challenge.data,
  });

  if (document.querySelector(".verbs-dativ-akkusativ__results")) {
    challenge.displayResults(window.konnektoren.result);
  } else {
    challenge.initialize();
  }
}

// Start the challenge
initializeChallenge();
