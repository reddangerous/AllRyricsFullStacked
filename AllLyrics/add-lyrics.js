 const addLyricForm = document.querySelector('#add-lyric-form');

addLyricForm.addEventListener('submit', event => {
  event.preventDefault();

  const formData = new FormData(addLyricForm);
  const requestBody = Object.fromEntries(formData.entries());
  requestBody.id = `${Date.now()}-${Math.random().toString(36).substr(2, 9)}`; // generate a unique id

  fetch('http://localhost:12345/music', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(requestBody),
  })
    .then(response => response.json())
    .then(data => {
      console.log('Lyric added:', data);
      addLyricForm.reset();
      loadLyricsTable();
    })
    .catch(error => console.error(error));
});
