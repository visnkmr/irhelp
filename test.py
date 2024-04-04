import plotly.graph_objects as go
import json
import dash
import dash_core_components as dcc
import dash_html_components as html
from dash.dependencies import Input, Output
# Assuming the JSON file is named 'data.json'
with open('3696.txt', 'r') as file:
    datas = json.load(file)
    # print(datas)
app = dash.Dash(__name__)
app.layout = html.Div([
    dcc.Slider(
        id='distance-slider',
        min=0,
        max=100, # Adjust the max value as needed
        step=1,
        value=50, # Default value
        marks={i: str(i) for i in range(0, 101, 10)}, # Optional: marks on the slider
    ),
    dcc.Graph(id='scatter-plot')
])
@app.callback(
    Output('scatter-plot', 'figure'),
    [Input('distance-slider', 'value')]
)
def update_graph(distance):
    # Your existing code to process the data
    data = []
    for item in datas:
        named = item['name'].strip('"')
        x, y = item['xycoord'].strip('()').split(',')
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