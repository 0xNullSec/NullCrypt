namespace Builder
{
    partial class Form1
    {
        private System.ComponentModel.IContainer components = null;

        protected override void Dispose(bool disposing)
        {
            if (disposing && (components != null))
            {
                components.Dispose();
            }
            base.Dispose(disposing);
        }

        #region Windows Form Designer generated code

        private void InitializeComponent()
        {
            this.components = new System.ComponentModel.Container();
            System.ComponentModel.ComponentResourceManager resources = new System.ComponentModel.ComponentResourceManager(typeof(Form1));
            this.guna2BorderlessForm1 = new Guna.UI2.WinForms.Guna2BorderlessForm(this.components);
            this.guna2DragControl1 = new Guna.UI2.WinForms.Guna2DragControl(this.components);
            this.pnlTop = new Guna.UI2.WinForms.Guna2Panel();
            this.btnMinimize = new Guna.UI2.WinForms.Guna2ControlBox();
            this.btnMaximize = new Guna.UI2.WinForms.Guna2ControlBox();
            this.btnClose = new Guna.UI2.WinForms.Guna2ControlBox();
            this.lblTitle = new Guna.UI2.WinForms.Guna2HtmlLabel();
            this.tlpContent = new System.Windows.Forms.TableLayoutPanel();
            this.pnlLeftCont = new System.Windows.Forms.Panel();
            this.btnBuild = new Guna.UI2.WinForms.Guna2Button();
            this.pnlInputs = new System.Windows.Forms.Panel();
            this.txtPayloadPath = new Guna.UI2.WinForms.Guna2TextBox();
            this.btnBrowsePayload = new Guna.UI2.WinForms.Guna2Button();
            this.pnlRightCont = new System.Windows.Forms.Panel();
            this.guna2VScrollBar1 = new Guna.UI2.WinForms.Guna2VScrollBar();
            this.txtConsole = new System.Windows.Forms.RichTextBox();
            this.guna2HScrollBar1 = new Guna.UI2.WinForms.Guna2HScrollBar();
            this.lblConsoleTitle = new Guna.UI2.WinForms.Guna2HtmlLabel();
            this.pnlBottom = new Guna.UI2.WinForms.Guna2Panel();
            this.guna2Separator1 = new Guna.UI2.WinForms.Guna2Separator();
            this.lblAuthor = new Guna.UI2.WinForms.Guna2HtmlLabel();
            this.pnlTop.SuspendLayout();
            this.tlpContent.SuspendLayout();
            this.pnlLeftCont.SuspendLayout();
            this.pnlInputs.SuspendLayout();
            this.pnlRightCont.SuspendLayout();
            this.pnlBottom.SuspendLayout();
            this.SuspendLayout();
            // 
            // guna2BorderlessForm1
            // 
            this.guna2BorderlessForm1.BorderRadius = 15;
            this.guna2BorderlessForm1.ContainerControl = this;
            this.guna2BorderlessForm1.DockIndicatorTransparencyValue = 0.6D;
            this.guna2BorderlessForm1.TransparentWhileDrag = true;
            // 
            // guna2DragControl1
            // 
            this.guna2DragControl1.DockIndicatorTransparencyValue = 0.6D;
            this.guna2DragControl1.TargetControl = this.pnlTop;
            this.guna2DragControl1.UseTransparentDrag = true;
            // 
            // pnlTop
            // 
            this.pnlTop.BackColor = System.Drawing.Color.FromArgb(((int)(((byte)(12)))), ((int)(((byte)(12)))), ((int)(((byte)(12)))));
            this.pnlTop.Controls.Add(this.btnMinimize);
            this.pnlTop.Controls.Add(this.btnMaximize);
            this.pnlTop.Controls.Add(this.btnClose);
            this.pnlTop.Controls.Add(this.lblTitle);
            this.pnlTop.Dock = System.Windows.Forms.DockStyle.Top;
            this.pnlTop.Location = new System.Drawing.Point(0, 0);
            this.pnlTop.Name = "pnlTop";
            this.pnlTop.Size = new System.Drawing.Size(858, 35);
            this.pnlTop.TabIndex = 0;
            // 
            // btnMinimize
            // 
            this.btnMinimize.Anchor = ((System.Windows.Forms.AnchorStyles)((System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right)));
            this.btnMinimize.ControlBoxType = Guna.UI2.WinForms.Enums.ControlBoxType.MinimizeBox;
            this.btnMinimize.FillColor = System.Drawing.Color.Transparent;
            this.btnMinimize.IconColor = System.Drawing.Color.White;
            this.btnMinimize.Location = new System.Drawing.Point(732, 0);
            this.btnMinimize.Name = "btnMinimize";
            this.btnMinimize.Size = new System.Drawing.Size(39, 35);
            this.btnMinimize.TabIndex = 3;
            // 
            // btnMaximize
            // 
            this.btnMaximize.Anchor = ((System.Windows.Forms.AnchorStyles)((System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right)));
            this.btnMaximize.ControlBoxType = Guna.UI2.WinForms.Enums.ControlBoxType.MaximizeBox;
            this.btnMaximize.FillColor = System.Drawing.Color.Transparent;
            this.btnMaximize.IconColor = System.Drawing.Color.White;
            this.btnMaximize.Location = new System.Drawing.Point(776, 0);
            this.btnMaximize.Name = "btnMaximize";
            this.btnMaximize.Size = new System.Drawing.Size(39, 35);
            this.btnMaximize.TabIndex = 2;
            // 
            // btnClose
            // 
            this.btnClose.Anchor = ((System.Windows.Forms.AnchorStyles)((System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right)));
            this.btnClose.FillColor = System.Drawing.Color.Transparent;
            this.btnClose.HoverState.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(192)))), ((int)(((byte)(0)))), ((int)(((byte)(0)))));
            this.btnClose.IconColor = System.Drawing.Color.White;
            this.btnClose.Location = new System.Drawing.Point(820, 0);
            this.btnClose.Name = "btnClose";
            this.btnClose.Size = new System.Drawing.Size(39, 35);
            this.btnClose.TabIndex = 1;
            // 
            // lblTitle
            // 
            this.lblTitle.BackColor = System.Drawing.Color.Transparent;
            this.lblTitle.Font = new System.Drawing.Font("Consolas", 11F, System.Drawing.FontStyle.Bold);
            this.lblTitle.ForeColor = System.Drawing.Color.Crimson;
            this.lblTitle.Location = new System.Drawing.Point(13, 9);
            this.lblTitle.Name = "lblTitle";
            this.lblTitle.Size = new System.Drawing.Size(147, 20);
            this.lblTitle.TabIndex = 4;
            this.lblTitle.Text = "NULLCRYPT - v2.0.0";
            // 
            // tlpContent
            // 
            this.tlpContent.ColumnCount = 2;
            this.tlpContent.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Absolute, 369F));
            this.tlpContent.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 100F));
            this.tlpContent.Controls.Add(this.pnlLeftCont, 0, 0);
            this.tlpContent.Controls.Add(this.pnlRightCont, 1, 0);
            this.tlpContent.Dock = System.Windows.Forms.DockStyle.Fill;
            this.tlpContent.Location = new System.Drawing.Point(0, 35);
            this.tlpContent.Name = "tlpContent";
            this.tlpContent.RowCount = 1;
            this.tlpContent.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Percent, 100F));
            this.tlpContent.Size = new System.Drawing.Size(858, 268);
            this.tlpContent.TabIndex = 1;
            // 
            // pnlLeftCont
            // 
            this.pnlLeftCont.Controls.Add(this.btnBuild);
            this.pnlLeftCont.Controls.Add(this.pnlInputs);
            this.pnlLeftCont.Dock = System.Windows.Forms.DockStyle.Fill;
            this.pnlLeftCont.Location = new System.Drawing.Point(0, 0);
            this.pnlLeftCont.Margin = new System.Windows.Forms.Padding(0);
            this.pnlLeftCont.Name = "pnlLeftCont";
            this.pnlLeftCont.Padding = new System.Windows.Forms.Padding(17, 26, 17, 17);
            this.pnlLeftCont.Size = new System.Drawing.Size(369, 268);
            this.pnlLeftCont.TabIndex = 0;
            // 
            // btnBuild
            // 
            this.btnBuild.Anchor = ((System.Windows.Forms.AnchorStyles)(((System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left) 
            | System.Windows.Forms.AnchorStyles.Right)));
            this.btnBuild.BorderRadius = 6;
            this.btnBuild.FillColor = System.Drawing.Color.Crimson;
            this.btnBuild.Font = new System.Drawing.Font("Consolas", 12F, System.Drawing.FontStyle.Bold);
            this.btnBuild.ForeColor = System.Drawing.Color.White;
            this.btnBuild.HoverState.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(180)))), ((int)(((byte)(30)))), ((int)(((byte)(30)))));
            this.btnBuild.Location = new System.Drawing.Point(17, 207);
            this.btnBuild.Name = "btnBuild";
            this.btnBuild.Size = new System.Drawing.Size(334, 43);
            this.btnBuild.TabIndex = 7;
            this.btnBuild.Text = "C O M P I L A R";
            this.btnBuild.Click += new System.EventHandler(this.btnBuild_Click);
            // 
            // pnlInputs
            // 
            this.pnlInputs.Anchor = ((System.Windows.Forms.AnchorStyles)(((System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Left) 
            | System.Windows.Forms.AnchorStyles.Right)));
            this.pnlInputs.Controls.Add(this.txtPayloadPath);
            this.pnlInputs.Controls.Add(this.btnBrowsePayload);
            this.pnlInputs.Location = new System.Drawing.Point(17, 87);
            this.pnlInputs.Name = "pnlInputs";
            this.pnlInputs.Size = new System.Drawing.Size(334, 35);
            this.pnlInputs.TabIndex = 2;
            // 
            // txtPayloadPath
            // 
            this.txtPayloadPath.Anchor = ((System.Windows.Forms.AnchorStyles)((((System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom) 
            | System.Windows.Forms.AnchorStyles.Left) 
            | System.Windows.Forms.AnchorStyles.Right)));
            this.txtPayloadPath.BorderColor = System.Drawing.Color.FromArgb(((int)(((byte)(45)))), ((int)(((byte)(45)))), ((int)(((byte)(45)))));
            this.txtPayloadPath.BorderRadius = 4;
            this.txtPayloadPath.Cursor = System.Windows.Forms.Cursors.IBeam;
            this.txtPayloadPath.DefaultText = "";
            this.txtPayloadPath.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(25)))), ((int)(((byte)(25)))), ((int)(((byte)(25)))));
            this.txtPayloadPath.Font = new System.Drawing.Font("Segoe UI", 9F);
            this.txtPayloadPath.ForeColor = System.Drawing.Color.White;
            this.txtPayloadPath.Location = new System.Drawing.Point(0, 0);
            this.txtPayloadPath.Name = "txtPayloadPath";
            this.txtPayloadPath.PlaceholderText = "Ruta del Payload a inyectar...";
            this.txtPayloadPath.ReadOnly = true;
            this.txtPayloadPath.SelectedText = "";
            this.txtPayloadPath.Size = new System.Drawing.Size(214, 35);
            this.txtPayloadPath.TabIndex = 1;
            // 
            // btnBrowsePayload
            // 
            this.btnBrowsePayload.Anchor = ((System.Windows.Forms.AnchorStyles)(((System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom) 
            | System.Windows.Forms.AnchorStyles.Right)));
            this.btnBrowsePayload.BorderRadius = 4;
            this.btnBrowsePayload.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(40)))), ((int)(((byte)(40)))), ((int)(((byte)(40)))));
            this.btnBrowsePayload.Font = new System.Drawing.Font("Segoe UI", 9F);
            this.btnBrowsePayload.ForeColor = System.Drawing.Color.White;
            this.btnBrowsePayload.Location = new System.Drawing.Point(223, 0);
            this.btnBrowsePayload.Name = "btnBrowsePayload";
            this.btnBrowsePayload.Size = new System.Drawing.Size(111, 35);
            this.btnBrowsePayload.TabIndex = 2;
            this.btnBrowsePayload.Text = "EXE / Shellcode";
            this.btnBrowsePayload.Click += new System.EventHandler(this.btnBrowsePayload_Click);
            // 
            // pnlRightCont
            // 
            this.pnlRightCont.Controls.Add(this.guna2VScrollBar1);
            this.pnlRightCont.Controls.Add(this.guna2HScrollBar1);
            this.pnlRightCont.Controls.Add(this.txtConsole);
            this.pnlRightCont.Controls.Add(this.lblConsoleTitle);
            this.pnlRightCont.Dock = System.Windows.Forms.DockStyle.Fill;
            this.pnlRightCont.Location = new System.Drawing.Point(369, 0);
            this.pnlRightCont.Margin = new System.Windows.Forms.Padding(0);
            this.pnlRightCont.Name = "pnlRightCont";
            this.pnlRightCont.Padding = new System.Windows.Forms.Padding(9, 9, 17, 17);
            this.pnlRightCont.Size = new System.Drawing.Size(489, 268);
            this.pnlRightCont.TabIndex = 1;
            // 
            // guna2VScrollBar1
            // 
            this.guna2VScrollBar1.BindingContainer = this.txtConsole;
            this.guna2VScrollBar1.BorderRadius = 4;
            this.guna2VScrollBar1.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(15)))), ((int)(((byte)(15)))), ((int)(((byte)(15)))));
            this.guna2VScrollBar1.InUpdate = false;
            this.guna2VScrollBar1.LargeChange = 10;
            this.guna2VScrollBar1.Location = new System.Drawing.Point(454, 30);
            this.guna2VScrollBar1.Name = "guna2VScrollBar1";
            this.guna2VScrollBar1.ScrollbarSize = 18;
            this.guna2VScrollBar1.Size = new System.Drawing.Size(18, 220);
            this.guna2VScrollBar1.TabIndex = 10;
            this.guna2VScrollBar1.ThumbColor = System.Drawing.Color.FromArgb(((int)(((byte)(45)))), ((int)(((byte)(45)))), ((int)(((byte)(45)))));
            this.guna2VScrollBar1.ThumbStyle = Guna.UI2.WinForms.Enums.ThumbStyle.Inset;
            // 
            // txtConsole
            // 
            this.txtConsole.Anchor = ((System.Windows.Forms.AnchorStyles)((((System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom) 
            | System.Windows.Forms.AnchorStyles.Left) 
            | System.Windows.Forms.AnchorStyles.Right)));
            this.txtConsole.BackColor = System.Drawing.Color.FromArgb(((int)(((byte)(12)))), ((int)(((byte)(12)))), ((int)(((byte)(12)))));
            this.txtConsole.BorderStyle = System.Windows.Forms.BorderStyle.None;
            this.txtConsole.Font = new System.Drawing.Font("Consolas", 9F);
            this.txtConsole.ForeColor = System.Drawing.Color.Silver;
            this.txtConsole.Location = new System.Drawing.Point(9, 30);
            this.txtConsole.Name = "txtConsole";
            this.txtConsole.ReadOnly = true;
            this.txtConsole.ScrollBars = System.Windows.Forms.RichTextBoxScrollBars.None;
            this.txtConsole.Size = new System.Drawing.Size(463, 220);
            this.txtConsole.TabIndex = 8;
            this.txtConsole.Text = "";
            this.txtConsole.WordWrap = false;
            // 
            // guna2HScrollBar1
            // 
            this.guna2HScrollBar1.BindingContainer = this.txtConsole;
            this.guna2HScrollBar1.BorderRadius = 4;
            this.guna2HScrollBar1.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(15)))), ((int)(((byte)(15)))), ((int)(((byte)(15)))));
            this.guna2HScrollBar1.InUpdate = false;
            this.guna2HScrollBar1.LargeChange = 10;
            this.guna2HScrollBar1.Location = new System.Drawing.Point(9, 232);
            this.guna2HScrollBar1.Name = "guna2HScrollBar1";
            this.guna2HScrollBar1.ScrollbarSize = 18;
            this.guna2HScrollBar1.Size = new System.Drawing.Size(463, 18);
            this.guna2HScrollBar1.TabIndex = 11;
            this.guna2HScrollBar1.ThumbColor = System.Drawing.Color.FromArgb(((int)(((byte)(45)))), ((int)(((byte)(45)))), ((int)(((byte)(45)))));
            this.guna2HScrollBar1.ThumbStyle = Guna.UI2.WinForms.Enums.ThumbStyle.Inset;
            // 
            // lblConsoleTitle
            // 
            this.lblConsoleTitle.BackColor = System.Drawing.Color.Transparent;
            this.lblConsoleTitle.Font = new System.Drawing.Font("Segoe UI Semibold", 9F, System.Drawing.FontStyle.Bold);
            this.lblConsoleTitle.ForeColor = System.Drawing.Color.Gray;
            this.lblConsoleTitle.Location = new System.Drawing.Point(9, 9);
            this.lblConsoleTitle.Name = "lblConsoleTitle";
            this.lblConsoleTitle.Size = new System.Drawing.Size(126, 17);
            this.lblConsoleTitle.TabIndex = 12;
            this.lblConsoleTitle.Text = "LOG DE OPERACIONES";
            // 
            // pnlBottom
            // 
            this.pnlBottom.Controls.Add(this.guna2Separator1);
            this.pnlBottom.Controls.Add(this.lblAuthor);
            this.pnlBottom.Dock = System.Windows.Forms.DockStyle.Bottom;
            this.pnlBottom.Location = new System.Drawing.Point(0, 303);
            this.pnlBottom.Name = "pnlBottom";
            this.pnlBottom.Size = new System.Drawing.Size(858, 52);
            this.pnlBottom.TabIndex = 2;
            // 
            // guna2Separator1
            // 
            this.guna2Separator1.Anchor = ((System.Windows.Forms.AnchorStyles)(((System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left) 
            | System.Windows.Forms.AnchorStyles.Right)));
            this.guna2Separator1.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(45)))), ((int)(((byte)(45)))), ((int)(((byte)(45)))));
            this.guna2Separator1.Location = new System.Drawing.Point(17, 9);
            this.guna2Separator1.Name = "guna2Separator1";
            this.guna2Separator1.Size = new System.Drawing.Size(824, 9);
            this.guna2Separator1.TabIndex = 9;
            // 
            // lblAuthor
            // 
            this.lblAuthor.Anchor = ((System.Windows.Forms.AnchorStyles)(((System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left) 
            | System.Windows.Forms.AnchorStyles.Right)));
            this.lblAuthor.AutoSize = false;
            this.lblAuthor.BackColor = System.Drawing.Color.Transparent;
            this.lblAuthor.Font = new System.Drawing.Font("Consolas", 9F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lblAuthor.ForeColor = System.Drawing.Color.DimGray;
            this.lblAuthor.Location = new System.Drawing.Point(17, 26);
            this.lblAuthor.Name = "lblAuthor";
            this.lblAuthor.Size = new System.Drawing.Size(824, 17);
            this.lblAuthor.TabIndex = 10;
            this.lblAuthor.Text = "Created by: 0xNullSec | IG: @0xNullSec | TG: @xNullSec | GH: 0xNullSec";
            this.lblAuthor.TextAlignment = System.Drawing.ContentAlignment.MiddleCenter;
            // 
            // Form1
            // 
            this.AutoScaleDimensions = new System.Drawing.SizeF(6F, 13F);
            this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
            this.BackColor = System.Drawing.Color.FromArgb(((int)(((byte)(18)))), ((int)(((byte)(18)))), ((int)(((byte)(18)))));
            this.ClientSize = new System.Drawing.Size(858, 355);
            this.Controls.Add(this.tlpContent);
            this.Controls.Add(this.pnlBottom);
            this.Controls.Add(this.pnlTop);
            this.FormBorderStyle = System.Windows.Forms.FormBorderStyle.None;
            this.Icon = ((System.Drawing.Icon)(resources.GetObject("$this.Icon")));
            this.Name = "Form1";
            this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
            this.Text = "NullCrypt";
            this.pnlTop.ResumeLayout(false);
            this.pnlTop.PerformLayout();
            this.tlpContent.ResumeLayout(false);
            this.pnlLeftCont.ResumeLayout(false);
            this.pnlInputs.ResumeLayout(false);
            this.pnlRightCont.ResumeLayout(false);
            this.pnlRightCont.PerformLayout();
            this.pnlBottom.ResumeLayout(false);
            this.ResumeLayout(false);

        }

        #endregion

        private Guna.UI2.WinForms.Guna2BorderlessForm guna2BorderlessForm1;
        private Guna.UI2.WinForms.Guna2DragControl guna2DragControl1;
        private Guna.UI2.WinForms.Guna2Panel pnlTop;
        private Guna.UI2.WinForms.Guna2ControlBox btnClose;
        private Guna.UI2.WinForms.Guna2ControlBox btnMaximize;
        private Guna.UI2.WinForms.Guna2ControlBox btnMinimize;
        private Guna.UI2.WinForms.Guna2HtmlLabel lblTitle;
        private System.Windows.Forms.TableLayoutPanel tlpContent;
        private System.Windows.Forms.Panel pnlLeftCont;
        private Guna.UI2.WinForms.Guna2Button btnBuild;
        private System.Windows.Forms.Panel pnlInputs;
        private Guna.UI2.WinForms.Guna2TextBox txtPayloadPath;
        private Guna.UI2.WinForms.Guna2Button btnBrowsePayload;
        private System.Windows.Forms.Panel pnlRightCont;
        private System.Windows.Forms.RichTextBox txtConsole;
        private Guna.UI2.WinForms.Guna2HtmlLabel lblConsoleTitle;
        private Guna.UI2.WinForms.Guna2Panel pnlBottom;
        private Guna.UI2.WinForms.Guna2Separator guna2Separator1;
        private Guna.UI2.WinForms.Guna2HtmlLabel lblAuthor;

        // Declaración de las barras de scroll Guna
        private Guna.UI2.WinForms.Guna2VScrollBar guna2VScrollBar1;
        private Guna.UI2.WinForms.Guna2HScrollBar guna2HScrollBar1;
    }
}