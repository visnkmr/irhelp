import plotly.graph_objects as go
import json
import dash
import dash_core_components as dcc
import dash_html_components as html
from dash.dependencies import Input, Output
# Assuming the JSON file is named 'data.json'
with open('selected.json', 'r') as file:
    datas = json.load(file)
    # print(datas)
app = dash.Dash(__name__)
app.layout = html.Div([
    dcc.Slider(
        id='distance-slider',
        min=0,
        max=1800, # Adjust the max value as needed
        step=1,
        value=50, # Default value
        marks={i: str(i) for i in range(0, 1800, 100)}, # Optional: marks on the slider
    ),
    dcc.Graph(id='scatter-plot')
])
@app.callback(
    Output('scatter-plot', 'figure'),
    [Input('distance-slider', 'value')]
)

# def sort_points_by_nearest(points):
#     # Sort points by x-coordinate
#     points.sort(key=lambda p: p.x)
    
#     # Divide and Conquer to find closest points
#     closest_points = find_closest_points(points)
    
#     # Merge and sort by y-coordinate
#     closest_points.sort(key=lambda p: p.y)
    
#     return closest_points

# def brute_force(points, n):
#     min_distance = float('inf')
#     for i in range(n):
#         for j in range(i+1, n):
#             distance = calculateDistance(points[i], points[j])
#             if distance < min_distance:
#                 min_distance = distance
#     return min_distance

# def calculateDistance(point1, point2):
#     # Assuming points are tuples (x, y)
#     x_diff = point1[0] - point2[0]
#     y_diff = point1[1] - point2[1]
#     return sqrt(x_diff**2 + y_diff**2)

# def find_closest_points(points):
#     if len(points) <= 3:
#         return brute_force(points)
    
#     mid = len(points) // 2
#     left_half = find_closest_points(points[:mid])
#     right_half = find_closest_points(points[mid:])
    
#     return merge_closest_points(left_half, right_half)

# def merge_closest_points(left, right):
#     merged = []
#     i = j = 0
#     while i < len(left) and j < len(right):
#         if left[i].y < right[j].y:
#             merged.append(left[i])
#             i += 1
#         else:
#             merged.append(right[j])
#             j += 1
#     merged.extend(left[i:])
#     merged.extend(right[j:])
#     return merged

def update_graph(distance):
    # Your existing code to process the data
    data = []
    for item in datas:
        named = item['name'].strip('"')
        x, y = item['xcoord'],item['ycoord']
        dist = item['distance']
        if dist < distance: # Use the slider value here
            data.append(go.Scatter(x=[float(x)], y=[float(y)], hovertemplate='<b>%{text}</b>',
                                    text=[named + " " + str(float(dist)) + " km"], mode='markers', name=named))

    # Create the layout for the plot
    layout = go.Layout(title='Cartesian Plot Example',
                       xaxis=dict(title='X Axis'),
                       yaxis=dict(title='Y Axis'))

    # Create the figure and add the traces and layout
    fig = go.Figure(data=data, layout=layout)

    return fig
if __name__ == '__main__':
    app.run_server(debug=True)