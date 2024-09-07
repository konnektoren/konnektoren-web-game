const endings = [
    { ending: '-chen', article: 'das' },
    { ending: '-lein', article: 'das' },
    { ending: '-icht', article: 'das' },
    { ending: '-il', article: 'das' },
    { ending: '-it', article: 'das' },
    { ending: '-ma', article: 'das' },
    { ending: '-ment', article: 'das' },
    { ending: '-tel', article: 'das' },
    { ending: '-tum', article: 'das' },
    { ending: '-um', article: 'das' },
    { ending: '-e', article: 'die' },
    { ending: '-anz', article: 'die' },
    { ending: '-enz', article: 'die' },
    { ending: '-ei', article: 'die' },
    { ending: '-ie', article: 'die' },
    { ending: '-heit', article: 'die' },
    { ending: '-keit', article: 'die' },
    { ending: '-ik', article: 'die' },
    { ending: '-sion', article: 'die' },
    { ending: '-tion', article: 'die' },
    { ending: '-sis', article: 'die' },
    { ending: '-tät', article: 'die' },
    { ending: '-ung', article: 'die' },
    { ending: '-ur', article: 'die' },
    { ending: '-schaft', article: 'die' },
    { ending: '-ant', article: 'der' },
    { ending: '-ast', article: 'der' },
    { ending: '-ich', article: 'der' },
    { ending: '-ig', article: 'der' },
    { ending: '-ling', article: 'der' },
    { ending: '-or', article: 'der' },
    { ending: '-us', article: 'der' },
    { ending: '-en', article: 'der' },
    { ending: '-el', article: 'der' },
    { ending: '-er', article: 'der' },
    { ending: '-in', article: 'die' },
    { ending: '-frau', article: 'die' }
];

let currentEndingIndex = 0;

function displayEnding() {
    const { ending } = endings[currentEndingIndex];
    const endingDisplayElement = document.getElementById('ending-display');
    endingDisplayElement.textContent = `Noun ending: ${ending}`;
    document.getElementById('correct-answer').textContent = '';
}

function checkAnswer(selectedArticle) {
    const correctArticle = endings[currentEndingIndex].article;
    const feedbackElement = document.getElementById('feedback');
    const correctAnswerElement = document.getElementById('correct-answer');

    if (selectedArticle === correctArticle) {
        document.body.className = 'green-bg';
        feedbackElement.textContent = 'Correct!';
    } else {
        document.body.className = 'red-bg';
        feedbackElement.textContent = 'Incorrect!';
    }

    correctAnswerElement.textContent = `The correct answer is: ${correctArticle}`;
    setTimeout(() => {
        document.body.className = '';
        currentEndingIndex = (currentEndingIndex + 1) % endings.length;
        displayEnding();
    }, 2000);
}

window.onload = displayEnding;
