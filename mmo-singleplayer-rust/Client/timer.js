class Timer {
    constructor(callback, timeInterval) {
        this.callback = callback;
        this.timeInterval = timeInterval;
    }

    start() {
        this.expected = Date.now() + this.timeInterval;
        this.timeout = setTimeout(this.round.bind(this), this.timeInterval);
    }

    stop() {
        clearTimeout(this.timeout);
    }

    round() {
        const drift = Date.now() - this.expected;
        this.callback();
        this.expected += this.timeInterval;
        this.timeout = setTimeout(this.round.bind(this), this.timeInterval - drift);
    }
}

module.exports = Timer;