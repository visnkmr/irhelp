import plotly.graph_objects as go
import json

# Assuming the JSON file is named 'data.json'
with open('3696.txt', 'r') as file:
    datas = json.load(file)
    # print(datas)

data=[]
nos=0;
for item in datas:
    named = item['name'].strip('"')
    x,y = item['xycoord'].strip('()').split(',')
    dist = item['distance']
    if(dist<50):
        # print((name))
        data.append(go.Scatter(x=[float(x)], y=[float(y)], mode='markers', name=named+str(float(dist)) ))
    nos=nos+1;
    # if(nos>8):
    #     break;

# Create the layout for the plot
layout = go.Layout(title='Cartesian Plot Example',
                   xaxis=dict(title='X Axis'),
                   yaxis=dict(title='Y Axis'))

# Create the figure and add the traces and layout
fig = go.Figure(data=data, layout=layout)

# Display the figure
fig.show()