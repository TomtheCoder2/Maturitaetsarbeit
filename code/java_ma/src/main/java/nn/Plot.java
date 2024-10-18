package nn;

import org.jfree.chart.ChartFactory;
import org.jfree.chart.ChartPanel;
import org.jfree.chart.JFreeChart;
import org.jfree.chart.plot.PlotOrientation;
import org.jfree.chart.title.TextTitle;
import org.jfree.data.xy.XYSeries;
import org.jfree.data.xy.XYSeriesCollection;
import org.jfree.ui.ApplicationFrame;
import org.jfree.ui.RefineryUtilities;

import javax.swing.*;
import java.util.List;

import static java.lang.Math.abs;

public class Plot extends ApplicationFrame {
    public static  JFreeChart chart;

    public Plot(String title) {
        super(title);
    }

    public static ApplicationFrame livePlot(List<XYSeries> seriesList, String title) {
        final XYSeriesCollection data = new XYSeriesCollection();
        for (XYSeries s : seriesList) {
            data.addSeries(s);
        }
        chart = ChartFactory.createXYLineChart(
                title,
                "epoch",
                title,
                data,
                PlotOrientation.VERTICAL,
                true,
                true, true);
        final ChartPanel chartPanel = new ChartPanel(chart);
        chartPanel.setPreferredSize(new java.awt.Dimension(1500, 700));
        var AF = new ApplicationFrame(title);

        AF.setContentPane(chartPanel);
        AF.pack();
        RefineryUtilities.centerFrameOnScreen(AF);
        AF.setExtendedState(JFrame.MAXIMIZED_BOTH);
//        AF.setUndecorated(true);
        AF.setVisible(true);
        return AF;
    }

    public static void changeEpochs(int epochs) {
        chart.clearSubtitles();
        chart.addSubtitle(new TextTitle("epoch: " + epochs));
    }

    public void plotAccuracy(List<Matrix> accuracy) {
        XYSeries series = new XYSeries("Accuracy");
        for (int i = 0; i < accuracy.size(); i += 1200) {
            float sum = 0;
            int j;
            for (j = 0; j < 1200; j++) {
                sum += abs(accuracy.get(i + j).data[0][0]);
            }
            series.add(i / 1200, sum / 1200);
        }
        final XYSeriesCollection data = new XYSeriesCollection();
        data.addSeries(series);
        final JFreeChart chart = ChartFactory.createXYLineChart(
                "Accuracy",
                "epoch",
                "accuracy",
                data,
                PlotOrientation.VERTICAL,
                true,
                true, true);
        final ChartPanel chartPanel = new ChartPanel(chart);
        chartPanel.setPreferredSize(new java.awt.Dimension(1500, 700));
        setContentPane(chartPanel);
    }

    public void plot(List<Matrix> errors, double[][] errorsPerColor) {
        final XYSeries series = new XYSeries("Training errors:");
        for (int i = 0; i < errors.size(); i++) {
            series.add(i, errors.get(i).data[0][0]);
        }

        List<XYSeries> seriesList = new java.util.ArrayList<>();
        for (int i = 0; i < errorsPerColor[0].length; i++) {
            seriesList.add(new XYSeries("Color " + i));
        }
        for (int i = 0; i < errorsPerColor.length; i++) {
            for (int j = 0; j < errorsPerColor[i].length; j++) {
                seriesList.get(j).add(i, errorsPerColor[i][j]);
            }
        }
        final XYSeriesCollection data = new XYSeriesCollection();
        data.addSeries(series);
        for (XYSeries s : seriesList) {
            data.addSeries(s);
        }
        final JFreeChart chart = ChartFactory.createXYLineChart(
                "Training errors",
                "epoch",
                "error %",
                data,
                PlotOrientation.VERTICAL,
                true,
                true,
                false
        );


        final ChartPanel chartPanel = new ChartPanel(chart);
        chartPanel.setPreferredSize(new java.awt.Dimension(1500, 700));
        setContentPane(chartPanel);
    }
}
