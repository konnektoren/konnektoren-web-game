class KonnektorenChallenge {
  constructor(config) {
    this.id = config.id;
    this.type = config.type;
    this.task_ids = config.task_ids;
    this.state = {
      currentIndex: 0,
      correctAnswers: 0,
      userAnswers: [],
      isFinished: false,
      startTime: new Date(),
      endTime: null,
    };
    this.data = config.data;
  }

  // Core methods
  initialize() {
    this.translateStaticText();
    this.loadQuestion();
    this.setupEventListeners();
    window.konnektoren.setState(this.state);
  }

  translateStaticText() {
    // Common translation logic
  }

  setupEventListeners() {
    // Common event listener setup logic
  }

  loadQuestion() {
    // Common question loading logic
  }

  checkAnswer(answer) {
    // Common answer validation logic
  }

  calculatePerformance() {
    return this.state.correctAnswers / this.data.get("questions").length;
  }

  finish() {
    this.state.endTime = new Date();
    this.state.isFinished = true;
    window.konnektoren.setState(this.state);

    console.log(this.state.userAnswers);

    const result = {
      id: this.id,
      performance: this.calculatePerformance(),
      data: {
        answers: this.state.userAnswers,
        timeSpent: this.state.endTime - this.state.startTime,
      },
    };

    window.konnektoren.executeCommand({
      type: "Challenge",
      action: "Finish",
      result,
    });
  }

  // Results handling
  displayResults() {
    // Common results display logic
  }
}

(function () {
  if (window.konnektoren) {
    console.log(
      "Konnektoren WASM implementation detected, skipping development implementation",
    );
    return;
  }

  window.konnektoren = {
    version: "dev",

    // Core data storage
    challenge: {},
    challengeState: {
      currentIndex: 0,
      correctAnswers: 0,
      userAnswers: [],
      isFinished: false,
      startTime: null,
      endTime: null,
    },
    result: {},
    i18n: {},

    // Challenge management
    createChallenge(config) {
      return new KonnektorenChallenge(config);
    },

    setState(newState) {
      this.challengeState = { ...this.challengeState, ...newState };
    },

    getState() {
      return this.challengeState;
    },

    // Translation system
    tr(key, params = {}) {
      const translation = this.i18n[key] || key;
      return Object.entries(params).reduce(
        (str, [key, value]) => str.replace(`{${key}}`, value),
        translation,
      );
    },

    // Command handling
    executeCommand(command) {
      console.log("Command executed:", command);

      if (command.type === "Challenge" && command.action === "Finish") {
        this.result = command.result;
        if (typeof this.onChallengeFinish === "function") {
          this.onChallengeFinish(command.result);
        }
      }
    },

    // Development helper functions
    dev: {
      setChallenge(challengeData) {
        window.konnektoren.challenge = challengeData;
        console.log("Challenge data set:", challengeData);
      },

      setI18n(i18nData) {
        window.konnektoren.i18n = i18nData;
        console.log("I18n data set:", i18nData);
      },

      setResult(resultData) {
        window.konnektoren.result = resultData;
        console.log("Result data set:", resultData);
      },

      onFinish(callback) {
        window.konnektoren.onChallengeFinish = callback;
      },

      logState() {
        console.log("Current Konnektoren State:", {
          challenge: window.konnektoren.challenge,
          challengeState: window.konnektoren.challengeState,
          result: window.konnektoren.result,
          i18n: window.konnektoren.i18n,
        });
      },

      // Development testing helpers
      simulateEvent(event) {
        window.konnektoren.sendEvent(event);
      },

      simulateCommand(command) {
        window.konnektoren.executeCommand(command);
      },
    },
  };

  console.log("Konnektoren development implementation initialized");
})();

if (typeof module !== "undefined" && module.exports) {
  module.exports = window.konnektoren;
}
