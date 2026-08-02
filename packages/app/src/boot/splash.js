(function () {
  var theme = 'dark';
  var accent = '#9184d9';

  try {
    var raw = localStorage.getItem('lightnotes:prefs:v1');
    if (raw) {
      var prefs = JSON.parse(raw);
      if (prefs.theme === 'Light') {
        theme = 'light';
      }
      if (typeof prefs.accent === 'string' && prefs.accent.length > 0) {
        accent = prefs.accent;
      }
    }
  } catch (error) {
    theme = 'dark';
  }

  var root = document.documentElement;
  root.setAttribute('data-theme', theme);
  root.style.setProperty('--accent', accent);
})();
