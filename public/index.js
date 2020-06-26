import init, {
    Othello
} from './pkg/othello_wasm_in_rust.js';

init('./pkg/othello_wasm_in_rust.wasm');

var othello = Othello.new();
async function run() {
    othello.draw();
    if (!othello.finish()) {
        othello.start(true, true);
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
setInterval(run, 200);
