class ZeitangabenChallenge extends KonnektorenChallenge {
  constructor(config) {
    super({
      id: config.id,
      type: "zeitangaben",
      data: config.data,
    });

    this.score = 0;
    this.elements = {
      title: document.querySelector(".zeitangaben__title"),
      description: document.querySelector(".zeitangaben__description"),
      progressBar: document.querySelector(".zeitangaben__progress-bar"),
      scenarioText: document.querySelector(".zeitangaben__scenario-text"),
      select: document.querySelector(".zeitangaben__select"),
      hintBox: document.querySelector(".zeitangaben__hint"),
      feedback: document.querySelector(".zeitangaben__feedback"),
      scoreDisplay: document.querySelector(".zeitangaben__score"),
      resultSummary: document.querySelector(".zeitangaben__result-summary"),
    };
  }

  translateStaticText() {
    this.elements.title.textContent = window.konnektoren.tr(
      "Time Expressions in German",
    );
    this.elements.description.textContent = window.konnektoren.tr(
      "Select the correct time expression",
    );
  }

  shuffleOptions(options) {
    return [...options].sort(() => Math.random() - 0.5);
  }

  loadQuestion() {
    if (this.state.currentIndex >= this.data.get("questions").length) {
      this.finish();
      return;
    }

    const question = this.data.get("questions")[this.state.currentIndex];
    const options = this.shuffleOptions(question.get("options"));

    this.elements.scenarioText.textContent = question.get("scenario");
    this.elements.select.innerHTML = `
      <option value="">${window.konnektoren.tr("Select...")}</option>
      ${options
        .map((option) => `<option value="${option}">${option}</option>`)
        .join("")}
    `;

    this.elements.hintBox.textContent = `💡 ${window.konnektoren.tr("Hint")}: ${question.get("hint")}`;
    this.elements.feedback.textContent = "";
    this.elements.feedback.className = "zeitangaben__feedback";
    this.elements.select.className = "zeitangaben__select";
    this.updateProgressBar();
  }

  checkAnswer(selectedOption) {
    const currentQuestion = this.data.get("questions")[this.state.currentIndex];
    const correctAnswer = currentQuestion.get("correctAnswer");
    const isCorrect = selectedOption === correctAnswer;

    if (!selectedOption) {
      this.showFeedback("please_select", "warning");
      return;
    }

    if (isCorrect) {
      this.score += 10;
      this.state.correctAnswers++;
      this.showFeedback("correct", "success");
      this.elements.select.classList.add("zeitangaben__select--correct");
      this.elements.scoreDisplay.textContent = `${window.konnektoren.tr("Score")}: ${this.score}`;

      setTimeout(() => {
        this.state.currentIndex++;
        this.loadQuestion();
      }, 1000);
    } else {
      this.showFeedback("incorrect", "error", correctAnswer);
      this.elements.select.classList.add("zeitangaben__select--incorrect");
    }

    this.state.userAnswers.push({
      questionId: this.state.currentIndex + 1,
      scenario: currentQuestion.get("scenario"),
      userAnswer: selectedOption,
      correctAnswer: correctAnswer,
      isCorrect: isCorrect,
    });
  }

  showFeedback(type, status, correctAnswer = "") {
    const messages = {
      correct: window.konnektoren.tr("Correct! Well done!"),
      incorrect: `${window.konnektoren.tr("Incorrect!")} ${window.konnektoren.tr("Correct answer")}: "${correctAnswer}"`,
      please_select: window.konnektoren.tr("Please select an option"),
    };

    this.elements.feedback.textContent = messages[type];
    this.elements.feedback.className = `zeitangaben__feedback zeitangaben__feedback--${status}`;
  }

  updateProgressBar() {
    const progress =
      ((this.state.currentIndex + 1) / this.data.get("questions").length) * 100;
    this.elements.progressBar.style.width = `${progress}%`;
  }

  setupEventListeners() {
    this.elements.select.addEventListener("change", (e) => {
      this.checkAnswer(e.target.value);
    });
  }

  finish() {
    const totalQuestions = this.data.get("questions").length;
    const performance = this.score / (totalQuestions * 10);

    this.elements.resultSummary.innerHTML = `
      <h2>${window.konnektoren.tr("Exercise Complete!")}</h2>
      <p>${window.konnektoren.tr("Score")}: ${this.score} ${window.konnektoren.tr("out of")} ${totalQuestions * 10}</p>
      <p>${window.konnektoren.tr("Correct Answers")}: ${this.state.correctAnswers} ${window.konnektoren.tr("out of")} ${totalQuestions}</p>
    `;

    // Hide exercise elements
    ["scenarioText", "select", "hintBox"].forEach((element) => {
      this.elements[element].classList.add("zeitangaben__hidden");
    });

    // Show results
    this.elements.resultSummary.classList.remove("zeitangaben__hidden");

    super.finish();
  }
}

// Initialize the challenge
function initializeChallenge() {
  const challenge = new ZeitangabenChallenge({
    id: "custom_zeitangaben",
    data: window.konnektoren.challenge.data,
  });

  if (document.querySelector(".zeitangaben__results")) {
    challenge.displayResults(window.konnektoren.result);
  } else {
    challenge.initialize();
  }
}

// Start the challenge
initializeChallenge();
