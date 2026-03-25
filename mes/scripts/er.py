import matplotlib.pyplot as plt
import matplotlib.patches as mpatches
import numpy as np

plt.rcParams['font.sans-serif'] = ['SimHei', 'DejaVu Sans']
plt.rcParams['axes.unicode_minus'] = False

fig, ax = plt.subplots(1, 1, figsize=(20, 14))

def draw_entity(ax, x, y, name, attributes, color='lightblue'):
    width = 2.8
    attr_height = 0.35
    header_height = 0.5
    total_height = header_height + len(attributes) * attr_height + 0.2
    
    rect = mpatches.FancyBboxPatch(
        (x - width/2, y - total_height/2),
        width, total_height,
        boxstyle="round,pad=0.02,rounding_size=0.1",
        facecolor=color,
        edgecolor='#333333',
        linewidth=1.5,
        alpha=0.9
    )
    ax.add_patch(rect)
    
    ax.text(x, y + total_height/2 - 0.3, name, ha='center', va='center', 
            fontsize=10, fontweight='bold', color='#1a1a1a')
    
    for i, attr in enumerate(attributes):
        attr_y = y + total_height/2 - 0.6 - i * attr_height
        ax.text(x - width/2 + 0.15, attr_y, attr, ha='left', va='center', 
                fontsize=7, color='#444444')
    
    return y - total_height/2

entities = {
    '产品': {
        'pos': (2, 11),
        'attrs': ['id: UUID', 'product_no: String', 'name: String', 'specification: String', 'unit: String'],
        'color': '#E8F5E9'
    },
    '工单': {
        'pos': (2, 6),
        'attrs': ['id: UUID', 'order_no: String', 'product_id: FK', 'quantity: Decimal', 'status: Enum', 'plan_start: DateTime'],
        'color': '#E3F2FD'
    },
    '工艺路线': {
        'pos': (6, 11),
        'attrs': ['id: UUID', 'product_id: FK', 'route_name: String', 'version: String', 'status: Enum'],
        'color': '#FFF3E0'
    },
    '工序': {
        'pos': (6, 6),
        'attrs': ['id: UUID', 'route_id: FK', 'process_name: String', 'sequence: Int', 'standard_time: Decimal'],
        'color': '#FFF3E0'
    },
    '排程计划': {
        'pos': (6, 1.5),
        'attrs': ['id: UUID', 'work_order_id: FK', 'process_id: FK', 'equipment_id: FK', 'start_time: DateTime'],
        'color': '#F3E5F5'
    },
    '物料': {
        'pos': (10, 11),
        'attrs': ['id: UUID', 'material_no: String', 'name: String', 'specification: String', 'unit: String'],
        'color': '#E0F7FA'
    },
    '库存': {
        'pos': (10, 6),
        'attrs': ['id: UUID', 'material_id: FK', 'warehouse_id: FK', 'batch_no: String', 'quantity: Decimal'],
        'color': '#E0F7FA'
    },
    '生产记录': {
        'pos': (14, 6),
        'attrs': ['id: UUID', 'work_order_id: FK', 'process_id: FK', 'quantity: Decimal', 'good_qty: Decimal'],
        'color': '#FCE4EC'
    },
    '检验单': {
        'pos': (14, 1.5),
        'attrs': ['id: UUID', 'work_order_id: FK', 'inspection_type: Enum', 'result: Enum', 'inspector_id: FK'],
        'color': '#FFEBEE'
    },
    '设备': {
        'pos': (18, 11),
        'attrs': ['id: UUID', 'equipment_no: String', 'name: String', 'type: String', 'status: Enum'],
        'color': '#F1F8E9'
    },
    '保养计划': {
        'pos': (18, 6),
        'attrs': ['id: UUID', 'equipment_id: FK', 'plan_type: Enum', 'plan_date: Date', 'executor_id: FK'],
        'color': '#F1F8E9'
    },
    '员工': {
        'pos': (18, 1.5),
        'attrs': ['id: UUID', 'employee_no: String', 'name: String', 'department_id: FK', 'status: Enum'],
        'color': '#FFFDE7'
    }
}

for name, info in entities.items():
    x, y = info['pos']
    draw_entity(ax, x, y, name, info['attrs'], info['color'])

relationships = [
    ('产品', '工单', '1:N'),
    ('产品', '工艺路线', '1:N'),
    ('工艺路线', '工序', '1:N'),
    ('工单', '排程计划', '1:N'),
    ('工序', '排程计划', '1:N'),
    ('物料', '库存', '1:N'),
    ('工单', '生产记录', '1:N'),
    ('工序', '生产记录', '1:N'),
    ('生产记录', '检验单', '1:N'),
    ('设备', '保养计划', '1:N'),
    ('设备', '排程计划', '1:N'),
    ('员工', '保养计划', '1:N'),
    ('员工', '检验单', '1:N'),
]

entity_positions = {name: info['pos'] for name, info in entities.items()}

for src, dst, rel in relationships:
    if src in entity_positions and dst in entity_positions:
        x1, y1 = entity_positions[src]
        x2, y2 = entity_positions[dst]
        
        ax.annotate('', xy=(x2, y2), xytext=(x1, y1),
                   arrowprops=dict(arrowstyle='->', color='#666666', lw=1.2))
        
        mid_x = (x1 + x2) / 2
        mid_y = (y1 + y2) / 2
        ax.text(mid_x, mid_y + 0.3, rel, ha='center', va='center', 
                fontsize=8, color='#333333',
                bbox=dict(boxstyle='round,pad=0.2', facecolor='white', edgecolor='none', alpha=0.8))

ax.set_xlim(-1, 21)
ax.set_ylim(-1, 13)
ax.set_aspect('equal')
ax.axis('off')

plt.title('MES系统核心实体关系图 (ER Diagram)', fontsize=16, fontweight='bold', pad=20)

er_image = '/root/source/rust/ithings/mes/docs/mes_er_diagram.png'
plt.savefig(er_image, dpi=200, bbox_inches='tight', facecolor='white', edgecolor='none')
print(f'ER图已保存: {er_image}')
