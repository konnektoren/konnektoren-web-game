class InterrogativeParticlesChallenge extends KonnektorenChallenge {
  constructor(config) {
    super({
      id: config.id,
      type: "interrogative_particles",
      data: config.data,
    });

    this.elements = {
      form: document.getElementById("quiz-form"),
      results: document.getElementById("results"),
      explanations: {},
    };
  }

  generateQuestions() {
    const questions = this.data.get("questions");
    const form = this.elements.form;

    form.innerHTML = "";

    questions.forEach((question) => {
      const questionDiv = document.createElement("div");
      questionDiv.className = "interrogative-particles__question";

      questionDiv.innerHTML = `
        <p class="interrogative-particles__text">
          ${question.get("id")}. ${question.get("sentence")}
          <select name="q${question.get("id")}" class="interrogative-particles__select">
            ${question
              .get("options")
              .map((option) => `<option value="${option}">${option}</option>`)
              .join("")}
          </select>
        </p>
        <div class="interrogative-particles__explanation" id="explanation-q${question.get("id")}"></div>
      `;

      form.appendChild(questionDiv);
      this.elements.explanations[`q${question.get("id")}`] =
        questionDiv.querySelector(`#explanation-q${question.get("id")}`);
    });

    const buttonsContainer = document.createElement("div");
    buttonsContainer.className = "interrogative-particles__buttons";

    const checkButton = document.createElement("button");
    checkButton.type = "submit";
    checkButton.className =
      "interrogative-particles__button interrogative-particles__button--check";
    checkButton.textContent = window.konnektoren.tr("Check Answers");

    const finishButton = document.createElement("button");
    finishButton.type = "button";
    finishButton.className =
      "interrogative-particles__button interrogative-particles__button--finish";
    finishButton.textContent = window.konnektoren.tr("Finish Exercise");
    finishButton.style.display = "none";

    buttonsContainer.appendChild(checkButton);
    buttonsContainer.appendChild(finishButton);
    form.appendChild(buttonsContainer);

    this.elements.checkButton = checkButton;
    this.elements.finishButton = finishButton;
  }

  checkAnswers() {
    const questions = this.data.get("questions");
    this.state.correctAnswers = 0;
    this.state.userAnswers = [];

    questions.forEach((question) => {
      const select = this.elements.form.querySelector(
        `select[name="q${question.get("id")}"]`,
      );
      const answer = select.value;
      const isCorrect = answer === question.get("correct_answer");

      if (isCorrect) {
        this.state.correctAnswers++;
      }

      this.state.userAnswers.push({
        question: question.get("sentence"),
        userAnswer: answer,
        correctAnswer: question.get("correct_answer"),
        isCorrect: isCorrect,
      });

      const explanationEl =
        this.elements.explanations[`q${question.get("id")}`];
      if (explanationEl) {
        const explanation = question.get("explanation");
        const explanationText = isCorrect
          ? explanation.get("correct")
          : explanation.get("wrong").get(answer);

        explanationEl.innerHTML = explanationText;
        explanationEl.className = `interrogative-particles__explanation ${
          isCorrect
            ? "interrogative-particles__explanation--correct"
            : "interrogative-particles__explanation--incorrect"
        }`;
      }
    });

    this.elements.results.innerHTML = window.konnektoren.tr(
      `Score: {score}/{total}`
        .replace("{score}", this.state.correctAnswers)
        .replace("{total}", questions.length),
    );

    this.elements.checkButton.disabled = true;
    this.elements.finishButton.style.display = "inline-block";
  }

  setupEventListeners() {
    this.elements.form.addEventListener("submit", (event) => {
      event.preventDefault();
      this.checkAnswers();
    });

    this.elements.finishButton.addEventListener("click", () => {
      this.finish();
    });
  }

  displayResults(result) {
    if (!this.elements.results) return;

    const answers = Array.from(result.data.answers);
    const correctCount = answers.filter((answer) => answer.isCorrect).length;
    const totalQuestions = answers.length;

    let resultsHtml = `
      <h2>${window.konnektoren.tr("Final Results")}</h2>
      <p>${window.konnektoren.tr("Score")}: ${correctCount}/${totalQuestions}</p>
      <p>${window.konnektoren.tr("Performance")}: ${(result.performance * 100).toFixed(1)}%</p>
      <p>${window.konnektoren.tr("Time Spent")}: ${Math.round((result.data.timeSpent || 0) / 1000)}s</p>
      <h3>${window.konnektoren.tr("Detailed Review")}</h3>
    `;

    answers.forEach((answer, index) => {
      resultsHtml += `
        <div class="interrogative-particles__result-item ${answer.isCorrect ? "interrogative-particles__result-item--correct" : "interrogative-particles__result-item--incorrect"}">
          <p><strong>${window.konnektoren.tr("Question")} ${index + 1}:</strong> ${answer.question}</p>
          <p>${window.konnektoren.tr("Your answer")}: ${answer.userAnswer}</p>
          <p>${window.konnektoren.tr("Correct answer")}: ${answer.correctAnswer}</p>
        </div>
      `;
    });

    this.elements.results.innerHTML = resultsHtml;
  }

  translateStaticText() {
    document.querySelector("h1").textContent = window.konnektoren.tr(
      "Interrogative Particles in German",
    );
    document.querySelector("header p").textContent = window.konnektoren.tr(
      "Select the correct interrogative particle to complete each sentence",
    );
  }

  initialize() {
    this.translateStaticText();
    this.generateQuestions();
    this.setupEventListeners();
    window.konnektoren.setState(this.state);
  }
}

// Initialize the challenge
function initializeChallenge() {
  const challenge = new InterrogativeParticlesChallenge({
    id: "custom_interrogative_particles",
    data: window.konnektoren.challenge.data,
  });

  if (document.querySelector(".results-container")) {
    challenge.displayResults(window.konnektoren.result);
  } else {
    challenge.initialize();
  }
}

// Start the challenge
initializeChallenge();
