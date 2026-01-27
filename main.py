import json
import matplotlib.pyplot as plt
import typer

app = typer.Typer(help="Plot a TSP tour from instance and result JSON files")

@app.command()
def plot_tsp(case_file: str, result_file: str):
    """
    Plot a TSP tour.

    Args:
        case_file: Path to the TSP instance JSON (cities + depot)
        result_file: Path to the tour JSON (list of city IDs)
    """
    # --- Load JSON files ---
    with open(case_file) as f:
        case = json.load(f)

    with open(result_file) as f:
        tour = json.load(f)

    # --- Prepare coordinates ---
    coords = {case['depot']['id']: (case['depot']['x'], case['depot']['y'])}
    for city in case['cities']:
        coords[city['id']] = (city['x'], city['y'])

    route_x = [coords[i][0] for i in tour]
    route_y = [coords[i][1] for i in tour]

    # --- Plot ---
    plt.figure(figsize=(6,6))
    plt.plot(route_x, route_y, '-o', color='blue', markersize=8)

    # Depot in red
    plt.plot(coords[case['depot']['id']][0], coords[case['depot']['id']][1], 'rs', markersize=10, label='Depot')

    plt.title(case['name'])
    plt.xlabel("X")
    plt.ylabel("Y")
    plt.grid(True)
    plt.axis('equal')
    plt.legend()
    plt.show()


if __name__ == "__main__":
    app()