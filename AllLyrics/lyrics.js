function loadLyricsTable() {
  const tableBody = document.querySelector('#lyrics-table tbody');

  // Clear existing data in the table
  tableBody.innerHTML = '';

  fetch('http://localhost:12345/music')
    .then(response => response.json())
    .then(data => {
      data.forEach(lyric => {
        const row = document.createElement('tr');
        row.dataset.id = lyric.id;
        row.innerHTML = `
          <td>${lyric.id}</td>
          <td>${lyric.music_title}</td>
          <td>${lyric.artist}</td>
          <td>${lyric.lyrics}</td>
          <td>
            <div class="form-check">
              <input type="checkbox" class="form-check-input delete-checkbox" data-id="${lyric.id}">
            </div>
          </td>
        `;
         tableBody.appendChild(row);

        row.addEventListener('click', () => {
          const selectedRow = document.querySelector('#lyrics-table tbody .table-primary');
          if (selectedRow) {
            selectedRow.classList.remove('table-primary');
          }
          row.classList.add('table-primary');
          // code to show the selected lyric in the form
        });
      });

      // Add event listener to the delete button
      const deleteButton = document.querySelector('#delete-selected-btn');
      deleteButton.addEventListener('click', () => {
        const selectedRows = document.querySelectorAll('#lyrics-table tbody .delete-checkbox:checked');
        selectedRows.forEach(selectedRow => {
          const id = selectedRow.dataset.id;
          fetch(`http://localhost:12345/music/${id}`, { method: 'DELETE' })
            .then(response => {
              if (response.ok) {
                const row = selectedRow.closest('tr');
                row.remove();
              } else {
                throw new Error('Error in response.');
              }
            })
            .catch(error => console.error(error));
        });
      });

      // Add event listener to the edit button
      const editButton = document.querySelector('#edit-lyrics-button');
      editButton.addEventListener('click', () => {
        const selectedRow = document.querySelector('#lyrics-table tbody .table-primary');
        if (selectedRow) {
          const id = selectedRow.dataset.id;
          // code to show the selected lyric in the form for editing
        } else {
          alert('Please select a row to edit.');
        }
      });
    })
    .catch(error => console.error(error));
}
