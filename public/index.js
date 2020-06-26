import init, {
    Othello
} from './pkg/othello_wasm_in_rust.js';

var othello;

async function run() {
    othello.draw();
    if (!othello.finish()) {
        othello.start(true, true, 5);
    } else {
        switch (othello.get_winner()) {
            case 1:
                alert("Black WIN!!");
                break;
            case -1:
                alert("White WIN!!");
                break;
            default:
                alert("DRAW!!");
        }

        othello = Othello.new();
    }
}

async function start() {
    await init();
    othello = Othello.new();
    setInterval(run, 200);
}
start();
